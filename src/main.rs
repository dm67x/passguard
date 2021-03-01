extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;
#[macro_use]
extern crate lazy_static;

mod database;
mod error;
mod model;

use error::FailureKind;
use model::{User, Password};
use rusqlite::params;
use simple_logger::SimpleLogger;

fn main() -> Result<(), FailureKind> {
    SimpleLogger::new().init()?;
    let user = User::find("06061998-64e4-4952-b70a-abf8983518d5")?;
    let passwords = user.passwords()?;
    println!("{:?} {:?}", user, passwords);
    Ok(())
}
