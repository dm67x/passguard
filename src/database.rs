use crate::error::FailureKind;
use rusqlite::{Connection, NO_PARAMS};

#[derive(Debug)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub password: String,
}

#[derive(Debug)]
pub struct Password {
    pub id: i64,
    pub website: String,
    pub password: String,
    pub user_id: i32,
}

pub fn db_exec<F>(f: F) -> Result<(), FailureKind> 
where F: Fn(Connection) -> Result<(), FailureKind>
{
    let conn = Connection::open("./passguard.db")?;
    conn.execute(
        r#"
            create table if not exists users (
                id integer primary key,
                name text not null,
                password text not null
            )
        "#,
        NO_PARAMS)?;
    conn.execute(
        r#"
            create table if not exists passwords (
                id integer primary key,
                website text not null,
                password text not null,
                user_id id integer not null,
                foreign key(user_id) references users(id)
            )
        "#,
        NO_PARAMS)?;
    f(conn)
}