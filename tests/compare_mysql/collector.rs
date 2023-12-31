use std::fs::read_to_string;

use scomp::compare_mysql::{build_deploy, output_column_info, output_index_info, read_config, start};
use scomp::init_log;
use scomp::value_object::Result;

static INIT: std::sync::Once = std::sync::Once::new();

#[test]
fn test_start() -> Result<()> {
    INIT.call_once(|| {
        init_log("Trace").unwrap();
    });
    let config_file = "datasource3.json";
    let columns_output_file = "diff-columns.csv";
    let indice_output_file = "diff-indices.csv";
    let source = false;
    let configs = read_config(config_file)?;

    start(&configs, source, columns_output_file, indice_output_file)?;

    Ok(())
}

#[test]
fn test_output_column_info() -> Result<()> {
    INIT.call_once(|| {
        init_log("Info").unwrap();
    });
    let configs = read_config("datasource3.json")?;
    let deploys = build_deploy(&configs)?;
    let columns_json = &read_to_string("columns.json")?;
    let mut columns = serde_json::from_str(columns_json)?;
    output_column_info("test-diff-columns.csv", &mut columns, &deploys)?;
    Ok(())
}

#[test]
fn test_output_index_info() -> Result<()> {
    INIT.call_once(|| {
        init_log("Info").unwrap();
    });
    let configs = read_config("datasource2.json")?;
    let deploys = build_deploy(&configs)?;
    let indices_json = &read_to_string("indices.json")?;
    let mut indices = serde_json::from_str(indices_json)?;
    output_index_info("test-diff-indices.csv", &mut indices, &deploys)?;
    Ok(())
}
