extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate magic_crypt;
extern crate hex;
extern crate sha2;

mod database;
mod encrypt;
mod error;
mod model;

use error::FailureKind;
use model::{Password, User};
use rusqlite::params;
use simple_logger::SimpleLogger;

fn main() -> Result<(), FailureKind> {
    SimpleLogger::new().init()?;
    let user = User::find("06061998-64e4-4952-b70a-abf8983518d5")?;
    let password = encrypt::encrypt(&user, "test")?;
    log::info!("Encrypted password: {}", password);
    let password = encrypt::decrypt(&user, password.as_str())?;
    log::info!("Decrypted password: {}", password);
    log::info!("Hashed password: {}", encrypt::hash(password.as_str())?);
    let passwords = user.passwords()?;
    println!("{:?} {:?}", user, passwords);
    Ok(())
}
