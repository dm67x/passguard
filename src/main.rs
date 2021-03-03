extern crate r2d2;
extern crate r2d2_sqlite;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate magic_crypt;
extern crate hex;
extern crate sha2;
extern crate warp;

mod api;
mod database;
mod encrypt;
mod error;
mod model;

use error::FailureKind;
use simple_logger::SimpleLogger;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), FailureKind> {
    SimpleLogger::new().init()?;
    database::get()?;
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));
    warp::serve(hello).run(([127, 0, 0, 1], 8080)).await;
    Ok(())
}
