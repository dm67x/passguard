use crate::error::FailureKind;
use lazy_static::lazy_static;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use std::sync::atomic::{AtomicBool, Ordering};

#[cfg(not(test))]
static DBPATH: &str = "passguard.db";
#[cfg(test)]
static DBPATH: &str = "passguard_test.db";

lazy_static! {
    static ref SQLITE: Pool<SqliteConnectionManager> = {
        let sqlite = SqliteConnectionManager::file(DBPATH);
        Pool::new(sqlite).unwrap()
    };
    static ref INITIALIZED: AtomicBool = AtomicBool::new(false);
}

fn init(pool: &Pool<SqliteConnectionManager>) -> Result<(), FailureKind> {
    pool.get()?.execute(
        r#"
        create table if not exists users (
            username text not null primary key,
            password text not null
        )
        "#,
        params![],
    )?;
    pool.get()?.execute(
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

pub(crate) fn get() -> Result<&'static Pool<SqliteConnectionManager>, FailureKind> {
    let pool = &*SQLITE;
    let initialized = INITIALIZED.swap(true, Ordering::SeqCst);
    if !initialized {
        init(pool)?;
    }
    Ok(pool)
}

#[cfg(test)]
pub(crate) fn empty() {
    let pool = get().unwrap().get().unwrap();
    pool.execute("DELETE FROM passwords", params![]).unwrap();
    pool.execute("DELETE FROM users", params![]).unwrap();
}
