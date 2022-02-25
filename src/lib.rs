pub mod compare_mysql;
pub mod value_object;

use std::str::FromStr;

use log::LevelFilter;
use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Root},
    Config,
};
use value_object::Result;

pub fn init_log(level: &str) -> Result<()> {
    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(
            Root::builder()
                .appender("stdout")
                .build(LevelFilter::from_str(level).unwrap_or(LevelFilter::Info)),
        )?;

    let _ = log4rs::init_config(config)?;
    Ok(())
}
