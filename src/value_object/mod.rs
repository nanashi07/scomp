mod models;

use std::result;

pub type Result<T> = result::Result<T, Box<dyn std::error::Error>>;
pub use models::*;
