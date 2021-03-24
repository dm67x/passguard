use crate::error::FailureKind;
use lazy_static::lazy_static;
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use std::sync::atomic::{AtomicBool, Ordering};

#[cfg(not(test))]
static DBPATH: &str = "passguard.db";

#[cfg(not(test))]
lazy_static! {
    static ref SQLITE: Pool<SqliteConnectionManager> = {
        let sqlite = SqliteConnectionManager::file(DBPATH);
        Pool::builder().build(sqlite).unwrap()
    };
}

#[cfg(test)]
lazy_static! {
    static ref SQLITE: Pool<SqliteConnectionManager> = {
        let sqlite = SqliteConnectionManager::memory();
        Pool::builder().build(sqlite).unwrap()
    };
}

lazy_static! {
    static ref INITIALIZED: AtomicBool = AtomicBool::new(false);
}

fn init(pool: &PooledConnection<SqliteConnectionManager>) -> Result<(), FailureKind> {
    pool.execute(
        r#"
        create table if not exists users (
            username text not null primary key,
            password text not null
        )
        "#,
        params![],
    )?;
    pool.execute(
        r#"
        create table if not exists passwords (
            id text primary key,
            url text not null,
            password text not null,
            user_id text not null,
            foreign key(user_id) references users(username)
        )
        "#,
        params![],
    )?;
    Ok(())
}

pub(crate) fn get() -> Result<PooledConnection<SqliteConnectionManager>, FailureKind> {
    let pool = (&*SQLITE).clone();
    let pool = pool.get()?;
    if !INITIALIZED.swap(true, Ordering::SeqCst) {
        init(&pool)?;
    }
    Ok(pool)
}

#[cfg(test)]
pub(crate) fn empty() {
    let pool = get().unwrap();
    pool.execute("DELETE FROM passwords", params![]).unwrap();
    pool.execute("DELETE FROM users", params![]).unwrap();
}
