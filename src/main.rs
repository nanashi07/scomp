use clap::{Arg, Command};
use log::{debug, info};
use std::time::Instant;

use scomp::value_object::Result;
use scomp::{
    compare_mysql::{read_config, start},
    init_log,
};

fn main() -> Result<()> {
    let cmd = command_args();
    let args = cmd.clone().get_matches();
    let config_file = args.value_of("config").unwrap();
    let columns_output_file = args.value_of("diff-columns").unwrap();
    let indice_output_file = args.value_of("diff-indices").unwrap();
    let level = args.value_of("level").unwrap();
    let source = args.is_present("source");

    init_log(level)?;

    debug!(
        "Args: config = {}, columns output = {}, indices output = {}, level = {}",
        config_file, columns_output_file, indice_output_file, level
    );
    let configs = read_config(config_file)?;

    let now = Instant::now();
    start(&configs, source, columns_output_file, indice_output_file)?;
    info!("Time elapsed {}s", now.elapsed().as_secs());

    Ok(())
}

/// Create command line arguments
fn command_args<'help>() -> Command<'help> {
    Command::new("scomp - Schema comparison for MySQL")
        .version("0.2.0")
        .author("Bruce Tsai")
        .args(&[
            Arg::new("config")
                .short('c')
                .long("config")
                .takes_value(true)
                .required(true)
                .help("MySQL connection config file"),
            Arg::new("diff-columns")
                .long("column")
                .takes_value(true)
                .default_value("diff-columns.csv")
                .help("Output file of columns comparison"),
            Arg::new("diff-indices")
                .long("indices")
                .takes_value(true)
                .default_value("diff-indices.csv")
                .help("Output file of indices comparison"),
            Arg::new("level")
                .long("level")
                .takes_value(true)
                .default_value("info")
                .help("Log level"),
            Arg::new("source").long("source").help("Output source data"),
        ])
}
