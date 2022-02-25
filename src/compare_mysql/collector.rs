use log::{debug, info, trace};
use mysql::{params, prelude::Queryable, Opts, Pool, PooledConn};
use std::{
    collections::{BTreeMap, HashMap},
    fs::{self, OpenOptions},
    include_str,
    io::{BufWriter, Write},
    path::Path,
    time::Instant,
    vec,
};

use crate::value_object::{
    ColumnInfo, ConnectInfo, Deploy, IndexInfo, Result, UniqueColumn, UniqueIndex,
};

pub fn start(
    configs: &Vec<ConnectInfo>,
    source: bool,
    columns_output_file: &str,
    indice_output_file: &str,
) -> Result<()> {
    let mut columns: Vec<ColumnInfo> = vec![];
    let mut indices: Vec<IndexInfo> = vec![];

    let count = configs.len();
    let mut iter = 0;

    for config in configs {
        let time = Instant::now();
        iter += 1;
        info!(
            "Collect info from {}/{} [mysql://{}:{}/{}]",
            iter, count, config.host, config.port, config.schema
        );

        // create database connection
        let connect_time = Instant::now();
        let url = format!(
            "mysql://{}:{}@{}:{}/{}",
            config.user, config.password, config.host, config.port, config.schema
        );
        let pool = Pool::new(Opts::from_url(&url)?)?;
        let mut conn = pool.get_conn()?;
        debug!(
            "Prepare connection elapsed {}s",
            connect_time.elapsed().as_secs()
        );

        // collect column data
        let mut set_of_columns = collect_column_info(&config, &mut conn)?;
        columns.append(&mut set_of_columns);
        // collect index data
        let mut set_of_indices = collect_index_info(&config, &mut conn)?;
        indices.append(&mut set_of_indices);

        debug!(
            "Collected from {}/{} [mysql://{}:{}/{}] elapsed {}s",
            iter,
            count,
            config.host,
            config.port,
            config.schema,
            time.elapsed().as_secs()
        );
    }

    if source {
        save_source(&columns, &indices)?;
    }

    let deploys = build_deploy(&configs)?;

    output_column_info(columns_output_file, &mut columns, &deploys)?;
    output_index_info(indice_output_file, &mut indices, &deploys)?;

    Ok(())
}

pub fn build_deploy(configs: &Vec<ConnectInfo>) -> Result<Vec<Deploy>> {
    let mut deploys: Vec<Deploy> = configs.iter().map(|c| c.to_deploy()).collect();
    deploys.dedup(); // make unique
    deploys.sort_by(|a, b| (a.environment, a.country).cmp(&(b.environment, b.country)));
    Ok(deploys)
}

fn save_source(columns: &Vec<ColumnInfo>, indices: &Vec<IndexInfo>) -> Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("columns.json")?;

    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, columns)?;

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("indices.json")?;

    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, indices)?;
    Ok(())
}

fn collect_column_info(config: &ConnectInfo, conn: &mut PooledConn) -> Result<Vec<ColumnInfo>> {
    let query = include_str!("../../resources/list-mysql-columns.sql");

    let stmt = conn.prep(&query)?;
    let result = conn.exec_map(
        stmt,
        params! { "schema" => config.schema.clone() },
        |mut row: mysql::Row| {
            trace!("{:?}", row);
            ColumnInfo {
                country: config.country.clone(),
                environment: config.environment.clone(),
                schema: config.schema.clone(),

                table: row.take("TABLE_NAME").unwrap(),
                column_name: row.take("COLUMN_NAME").unwrap(),
                column_type: row.take("COLUMN_TYPE").unwrap(),
                data_type: row.take("DATA_TYPE").unwrap(),
                text_max_length: row.take("CHARACTER_MAXIMUM_LENGTH").unwrap(),
                text_octet_length: row.take("CHARACTER_OCTET_LENGTH").unwrap(),
                num_precision: row.take("NUMERIC_PRECISION").unwrap(),
                default_value: row.take("COLUMN_DEFAULT").unwrap(),
                nullable: row.take("IS_NULLABLE").unwrap(),
                num_scale: row.take("NUMERIC_SCALE").unwrap(),
                comment: row.take("COLUMN_COMMENT").unwrap(),

                display: Option::None,
            }
        },
    )?;

    Ok(result)
}

fn collect_index_info(config: &ConnectInfo, conn: &mut PooledConn) -> Result<Vec<IndexInfo>> {
    let query = include_str!("../../resources/list-mysql-indices.sql");

    let stmt = conn.prep(&query)?;
    let result = conn.exec_map(
        stmt,
        params! { "schema" => config.schema.clone() },
        |mut row: mysql::Row| {
            trace!("{:?}", row);
            IndexInfo {
                country: config.country.clone(),
                environment: config.environment.clone(),
                schema: config.schema.clone(),

                table: row.take("TABLE_NAME").unwrap(),
                index_name: row.take("INDEX_NAME").unwrap(),
                unique: row.take("NON_UNIQUE").unwrap(),
                sequence: row.take("SEQ_IN_INDEX").unwrap(),
                column: row.take("COLUMN_NAME").unwrap(),

                display: Option::None,
            }
        },
    )?;

    Ok(result)
}

pub fn output_column_info(
    file: &str,
    columns: &Vec<ColumnInfo>,
    deploys: &Vec<Deploy>,
) -> Result<()> {
    debug!("Generating csv for {} columns", columns.len());

    let groups = columns.into_iter().fold(
        BTreeMap::new(),
        |mut acc: BTreeMap<UniqueColumn, Vec<&ColumnInfo>>, c| {
            acc.entry(UniqueColumn {
                schema: &c.schema,
                table: &c.table,
                column_name: &c.column_name,
            })
            .or_default()
            .push(c);
            acc
        },
    );

    let mut output = String::new();

    // header line
    output += "Schema\tTable\tColumn";
    for deploy in deploys {
        output += format!("\t[{} {}]", deploy.environment, deploy.country).as_str();
    }
    output += "\n";

    for (key, list) in groups {
        trace!("unique column: {:?}", key);
        output += format!("{}\t{}\t{}", key.schema, key.table, key.column_name).as_str();

        let most_used = find_most_used_column(&list, deploys.len() as i16);

        for deploy in deploys {
            if let Some(item) = list
                .iter()
                .find(|c| c.environment == deploy.environment && c.country == deploy.country)
            {
                let format = column_format(item);
                if most_used == format {
                    output += "\t";
                } else {
                    output += "\t**";
                }
                output += format.as_str();
            } else {
                if most_used.is_empty() {
                    output += "\t";
                } else {
                    output += "\t**";
                }
            }
        }
        output += "\n";
    }

    if !Path::new(file).parent().unwrap().exists() {
        fs::create_dir_all(Path::new(file).parent().unwrap())?;
    }

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(file)?;

    let mut writer = BufWriter::new(file);
    writer.write_all(output.as_bytes())?;

    Ok(())
}

fn column_format(item: &ColumnInfo) -> String {
    format!(
        "{}{} {}",
        item.column_type,
        if item.nullable == "YES" { " NULL" } else { "" },
        item.default_value.as_ref().unwrap_or(&String::new())
    )
    .trim()
    .to_string()
}

fn find_most_used_column(list: &Vec<&ColumnInfo>, deploy_count: i16) -> String {
    let counted_map = list.into_iter().map(|item| column_format(item)).fold(
        HashMap::new(),
        |mut acc: HashMap<String, i16>, item| {
            let counter = acc.entry(item).or_insert(0);
            *counter += 1;
            acc
        },
    );

    select_max_one(counted_map, list.len() as i16, deploy_count)
}

fn select_max_one(
    counted_map: HashMap<String, i16>,
    value_count: i16,
    deploy_count: i16,
) -> String {
    let result = &mut String::new();
    let mut count = 0;
    for (k, v) in counted_map {
        if count < v {
            *result = k;
            count = v;
        }
    }

    // deploy_count - value_count = whitespace cell count
    if deploy_count - value_count > count {
        String::new()
    } else {
        result.to_string()
    }
}

pub fn output_index_info(
    file: &str,
    indices: &Vec<IndexInfo>,
    deploys: &Vec<Deploy>,
) -> Result<()> {
    debug!("Generating csv for {} indices", indices.len());

    let groups = indices.iter().fold(
        BTreeMap::new(),
        |mut acc: BTreeMap<UniqueIndex, Vec<&IndexInfo>>, i| {
            acc.entry(UniqueIndex {
                schema: &i.schema,
                table: &i.table,
                index_name: &i.index_name,
            })
            .or_default()
            .push(i);
            acc
        },
    );

    let mut output = String::new();

    // header line
    output += "Schema\tTable\tIndex";
    for deploy in deploys {
        output += format!("\t[{} {}]", deploy.environment, deploy.country).as_str();
    }
    output += "\n";

    for (key, list) in groups {
        trace!("unique index: {:?}", key);
        output += format!("{}\t{}\t{}", key.schema, key.table, key.index_name).as_str();

        // find most used
        let most_used = find_most_used_index(deploys, &list);

        for deploy in deploys {
            let mut columns: Vec<&IndexInfo> = list
                .iter()
                .filter(|i| i.environment == deploy.environment && i.country == deploy.country)
                .map(|i| *i)
                .collect();

            output += "\t";
            if columns.len() > 0 {
                columns.sort_by(|a, b| a.sequence.cmp(&b.sequence));
                let format = index_format(&columns);
                if format != most_used {
                    output += "**";
                }
                output += format.as_str();
            } else {
                if !most_used.is_empty() {
                    output += "**";
                }
            }
        }
        output += "\n";
    }

    if !Path::new(file).parent().unwrap().exists() {
        fs::create_dir_all(Path::new(file).parent().unwrap())?;
    }

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(file)?;

    let mut writer = BufWriter::new(file);
    writer.write_all(output.as_bytes())?;

    Ok(())
}

fn index_format(columns: &Vec<&IndexInfo>) -> String {
    let unique = if columns[0].unique { "UNIQUE " } else { "" };
    let values = columns
        .iter()
        .map(|c| c.column.as_str())
        .collect::<Vec<&str>>()
        .join(", ");
    format!("{}{}", unique, values)
}

fn find_most_used_index(deploys: &Vec<Deploy>, list: &Vec<&IndexInfo>) -> String {
    let counted_map = deploys
        .iter()
        .map(|deploy| {
            let mut columns: Vec<&IndexInfo> = list
                .iter()
                .filter(|i| i.environment == deploy.environment && i.country == deploy.country)
                .map(|i| *i)
                .collect();

            if columns.len() == 0 {
                String::new()
            } else {
                columns.sort_by(|a, b| a.sequence.cmp(&b.sequence));
                index_format(&columns)
            }
        })
        .filter(|item| !item.is_empty())
        .fold(HashMap::new(), |mut acc: HashMap<String, i16>, item| {
            let value = acc.entry(item).or_insert(0);
            *value += 1;
            acc
        });

    select_max_one(counted_map, list.len() as i16, deploys.len() as i16)
}

// Read MySQL connection configuration from file
pub fn read_config(config_path: &str) -> Result<Vec<ConnectInfo>> {
    // TODO: check file exists
    let json_config = &fs::read_to_string(config_path)?;

    let config: Vec<ConnectInfo> = serde_json::from_str(json_config)?;

    Ok(config)
}
