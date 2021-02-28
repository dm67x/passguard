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
    let pool = &*database::SQLITE;
    let pool = pool.get()?;
    let mut prepare = pool.prepare("SELECT * FROM users LIMIT 1")?;
    let result: User = prepare.query_row(params![], |row| {
        Ok(User {
            id: row.get_unwrap(0),
            name: row.get_unwrap(1),
            password: row.get_unwrap(2),
        })
    })?;
    log::info!("Username: {:?}", result);
    Ok(())
}
