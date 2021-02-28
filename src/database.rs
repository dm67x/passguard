use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;

lazy_static! {
    pub static ref SQLITE: Pool<SqliteConnectionManager> = {
        let sqlite = SqliteConnectionManager::file("passguard.db");
        let pool = Pool::new(sqlite).unwrap();
        pool.get()
            .unwrap()
            .execute(
                r#"
                create table if not exists users (
                    id integer primary key,
                    name text not null,
                    password text not null
                )
                "#,
                params![],
            )
            .unwrap();
        pool.get()
            .unwrap()
            .execute(
                r#"
                create table if not exists passwords (
                    id integer primary key,
                    website text not null,
                    password text not null,
                    user_id id integer not null,
                    foreign key(user_id) references users(id)
                )
                "#,
                params![],
            )
            .unwrap();
        pool
    };
}
