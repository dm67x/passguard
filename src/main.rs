use error::FailureKind;
use rusqlite::NO_PARAMS;
use database::User;

mod database;
mod error;

fn main() -> Result<(), FailureKind> {
    database::db_exec(|conn| {
        let mut stmt = conn.prepare("SELECT * FROM users")?;
        let users = stmt.query_map(NO_PARAMS, |row| {
            Ok(User {
                id: row.get(0)?,
                name: row.get(1)?,
                password: row.get(2)?,
            })
        })?;
        for user in users {
            println!("Found user {:?}", user.unwrap());
        }
        Ok(())
    })?;
    Ok(())
}
