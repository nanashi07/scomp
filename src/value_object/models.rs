use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ConnectInfo {
    pub host: String,
    pub port: i32,
    pub schema: String,
    pub user: String,
    pub password: String,
    pub country: String,
    pub environment: String,
}

impl ConnectInfo {
    pub fn to_deploy(&self) -> Deploy {
        Deploy {
            country: &self.country,
            environment: &self.environment,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub struct Deploy<'a> {
    pub country: &'a str,
    pub environment: &'a str,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Serialize, Deserialize)]
pub struct ColumnInfo {
    pub country: String,
    pub environment: String,
    pub schema: String,

    pub table: String,
    pub column_name: String,
    pub column_type: String,
    pub data_type: String,
    pub text_max_length: Option<i32>,
    pub text_octet_length: Option<i32>,
    pub num_precision: Option<i32>,
    pub num_scale: Option<i32>,
    pub default_value: Option<String>,
    pub nullable: String,
    pub comment: Option<String>,

    pub display: Option<String>,
}

impl Ord for ColumnInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (
            &self.schema,
            &self.table,
            &self.column_name,
            &self.environment,
            &self.country,
        )
            .cmp(&(
                &other.schema,
                &other.table,
                &other.column_name,
                &other.environment,
                &other.country,
            ))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
pub struct IndexInfo {
    pub country: String,
    pub environment: String,
    pub schema: String,

    pub table: String,
    pub index_name: String,
    pub unique: bool,
    pub sequence: i32,
    pub column: String,

    pub display: Option<String>,
}

impl Ord for IndexInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (
            &self.schema,
            &self.table,
            &self.index_name,
            &self.environment,
            &self.country,
        )
            .cmp(&(
                &other.schema,
                &other.table,
                &other.index_name,
                &other.environment,
                &other.country,
            ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct UniqueColumn<'a> {
    pub schema: &'a str,
    pub table: &'a str,
    pub column_name: &'a str,
}

impl Ord for UniqueColumn<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (&self.schema, &self.table, &self.column_name).cmp(&(
            &other.schema,
            &other.table,
            &other.column_name,
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct UniqueIndex<'a> {
    pub schema: &'a str,
    pub table: &'a str,
    pub index_name: &'a str,
}

impl Ord for UniqueIndex<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (&self.schema, &self.table, &self.index_name).cmp(&(
            &other.schema,
            &other.table,
            &other.index_name,
        ))
    }
}
