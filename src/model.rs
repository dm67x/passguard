use crate::{database, error::FailureKind};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub password: String,
}

impl User {
    pub fn new(name: &str, password: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            password: password.to_string(),
        }
    }

    pub fn save(&self) -> Result<Self, FailureKind> {
        let pool = database::get()?.get()?;
        pool.execute("DELETE FROM users WHERE id = ?1", params![self.id])?;
        pool.execute(
            "INSERT INTO users VALUES(?1, ?2, ?3)",
            params![self.id, self.name, self.password],
        )?;
        Ok(self.clone())
    }

    pub fn destroy(&self) -> Result<(), FailureKind> {
        let pool = database::get()?.get()?;
        pool.execute("DELETE FROM users WHERE id = ?1", params![self.id])?;
        Ok(())
    }

    pub fn find_by(id: &str) -> Result<Self, FailureKind> {
        let pool = database::get()?.get()?;
        pool.query_row("SELECT * from users WHERE id = ?1", params![id], |row| {
            Ok(User {
                id: row.get(0)?,
                name: row.get(1)?,
                password: row.get(2)?,
            })
        })
        .map_err(|err| err.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Password {
    pub id: String,
    pub website: String,
    pub password: String,
    pub user_id: Option<String>,
}

impl Password {
    pub fn new(website: &str, password: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            website: website.to_string(),
            password: password.to_string(),
            user_id: None,
        }
    }

    pub fn save(&self) -> Result<Self, FailureKind> {
        let pool = database::get()?.get()?;
        pool.execute("DELETE FROM passwords WHERE id = ?1", params![self.id])?;
        pool.execute(
            "INSERT INTO passwords(id, website, password, user_id) VALUES (?1, ?2, ?3, ?4)",
            params![
                self.id,
                self.website,
                self.password,
                self.user_id.as_ref().unwrap_or(&"".to_string())
            ],
        )?;
        Ok(self.clone())
    }

    pub fn destroy(&self) -> Result<(), FailureKind> {
        let pool = database::get()?.get()?;
        pool.execute("DELETE FROM passwords WHERE id = ?1", params![self.id])?;
        Ok(())
    }

    pub fn find_by(id: &str) -> Result<Self, FailureKind> {
        let pool = database::get()?.get()?;
        let mut statement = pool.prepare("SELECT * FROM passwords WHERE id = ?1")?;
        statement
            .query_row(params![id], |row| {
                Ok(Password {
                    id: row.get(0)?,
                    website: row.get(1)?,
                    password: row.get(2)?,
                    user_id: row.get(3)?,
                })
            })
            .map_err(|err| err.into())
    }

    pub fn match_all(user: &User, website: &str) -> Result<Vec<Self>, FailureKind> {
        let pool = database::get()?.get()?;
        let mut statement =
            pool.prepare("SELECT * FROM passwords WHERE website = ?1 AND user_id = ?2")?;
        let mut rows = statement.query(params![website, user.id])?;
        let mut passwords = vec![];
        while let Some(row) = rows.next()? {
            passwords.push(Password {
                id: row.get(0)?,
                website: row.get(1)?,
                password: row.get(2)?,
                user_id: row.get(3)?,
            });
        }
        Ok(passwords)
    }
}
