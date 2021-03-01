use crate::{database, error::FailureKind};
use rusqlite::params;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct Password {
    pub id: String,
    pub website: String,
    pub password: String,
    pub user_id: String,
}

impl User {
    pub fn new(name: &str, password: &str) -> Result<Self, FailureKind> {
        let uuid = Uuid::new_v4().to_string();
        let pool = database::get()?.get()?;
        pool.execute(
            "INSERT INTO users VALUES (?1, ?2, ?3)",
            params![uuid, name, password],
        )?;
        User::find(&uuid)
    }

    pub fn destroy(&self) -> Result<(), FailureKind> {
        let pool = database::get()?.get()?;
        pool.execute("DELETE FROM users WHERE id = ?1", params![self.id])?;
        Ok(())
    }

    pub fn update(&self, name: &str, password: &str) -> Result<Self, FailureKind> {
        let pool = database::get()?.get()?;
        pool.execute(
            "UPDATE users SET name = ?2, password = ?3 WHERE id = ?1",
            params![self.id, name, password],
        )?;
        User::find(&self.id)
    }

    pub fn find(id: &str) -> Result<Self, FailureKind> {
        let pool = database::get()?.get()?;
        let user = pool.query_row("SELECT * from users WHERE id = ?1", params![id], |row| {
            Ok(User {
                id: row.get(0)?,
                name: row.get(1)?,
                password: row.get(2)?,
            })
        })?;
        Ok(user)
    }

    pub fn passwords(&self) -> Result<Vec<Password>, FailureKind> {
        let pool = database::get()?.get()?;
        let mut req = pool.prepare("SELECT * FROM passwords WHERE user_id = ?1")?;
        let passwords = req.query_map(params![self.id], |row| {
            Ok(Password {
                id: row.get(0)?,
                website: row.get(1)?,
                password: row.get(2)?,
                user_id: row.get(3)?,
            })
        });

        let mut results: Vec<Password> = vec![];
        for password in passwords? {
            results.push(password?);
        }
        Ok(results)
    }
}

impl Password {
    pub fn new(website: &str, password: &str, user_id: &str) -> Result<Self, FailureKind> {
        let uuid = Uuid::new_v4().to_string();
        let pool = database::get()?.get()?;
        pool.execute(
            "INSERT INTO passwords(id, website, password, user_id) VALUES (?1, ?2, ?3, ?4)",
            params![uuid, website, password, user_id],
        )?;
        let password = pool.query_row(
            "SELECT * from passwords WHERE id = ?1",
            params![uuid],
            |row| {
                Ok(Password {
                    id: row.get(0)?,
                    website: row.get(1)?,
                    password: row.get(2)?,
                    user_id: row.get(3)?,
                })
            },
        )?;
        Ok(password)
    }

    pub fn destroy(&self) -> Result<(), FailureKind> {
        let pool = database::get()?.get()?;
        pool.execute("DELETE FROM passwords WHERE id = ?1", params![self.id])?;
        Ok(())
    }

    pub fn update(&self, website: &str, password: &str) -> Result<Self, FailureKind> {
        let pool = database::get()?.get()?;
        pool.execute(
            "UPDATE passwords SET website = ?2, password = ?3 WHERE id = ?1",
            params![self.id, website, password],
        )?;
        let password = pool.query_row(
            "SELECT * from passwords WHERE id = ?1",
            params![self.id],
            |row| {
                Ok(Password {
                    id: row.get(0)?,
                    website: row.get(1)?,
                    password: row.get(2)?,
                    user_id: row.get(3)?,
                })
            },
        )?;
        Ok(password)
    }
}
