extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;
#[macro_use]
extern crate lazy_static;

mod database;
mod error;
mod model;

use error::FailureKind;
use model::User;
use rusqlite::params;
use simple_logger::SimpleLogger;

fn main() -> Result<(), FailureKind> {
    SimpleLogger::new().init()?;
    Ok(())
}
