use crate::error::FailureKind;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use std::sync::atomic::{AtomicBool, Ordering};

lazy_static! {
    static ref SQLITE: Pool<SqliteConnectionManager> = {
        let sqlite = SqliteConnectionManager::file("passguard.db");
        Pool::new(sqlite).unwrap()
    };
    static ref INITIALIZED: AtomicBool = AtomicBool::new(false);
}

fn init(pool: &Pool<SqliteConnectionManager>) -> Result<(), FailureKind> {
    pool.get()?.execute(
        r#"
        create table if not exists users (
            id text primary key,
            name text not null,
            password text not null
        )
        "#,
        params![],
    )?;
    pool.get()?.execute(
        r#"
        create table if not exists passwords (
            id text primary key,
            website text not null,
            password text not null,
            user_id text not null,
            foreign key(user_id) references users(id)
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
