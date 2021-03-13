use crate::{database, error::FailureKind};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub(crate) trait Model {
    fn save(&self) -> Result<Self, FailureKind>
    where
        Self: Sized;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct User {
    pub username: String,
    pub password: String,
}

impl User {
    pub(crate) fn new(username: &str, password: &str) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
        }
    }

    pub(crate) fn destroy(&self) -> Result<(), FailureKind> {
        let pool = database::get()?.get()?;
        pool.execute(
            "DELETE FROM users WHERE username = ?1",
            params![self.username],
        )?;
        Ok(())
    }

    pub(crate) fn find_by(username: &str) -> Result<Self, FailureKind> {
        let pool = database::get()?.get()?;
        pool.query_row(
            "SELECT * from users WHERE username = ?1",
            params![username],
            |row| {
                Ok(User {
                    username: row.get(0)?,
                    password: row.get(1)?,
                })
            },
        )
        .map_err(|err| err.into())
    }
}

impl Model for User {
    fn save(&self) -> Result<Self, FailureKind> {
        let pool = database::get()?.get()?;
        pool.execute(
            "DELETE FROM users WHERE username = ?1",
            params![self.username],
        )?;
        pool.execute(
            "INSERT INTO users VALUES(?1, ?2)",
            params![self.username, self.password],
        )?;
        Ok(self.clone())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Password {
    pub id: String,
    pub url: String,
    pub password: String,
    pub user_id: String,
}

impl Password {
    pub(crate) fn new(user: &User, url: &str, password: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            url: url.to_string(),
            password: password.to_string(),
            user_id: user.username.to_string(),
        }
    }

    pub(crate) fn destroy(&self) -> Result<(), FailureKind> {
        let pool = database::get()?.get()?;
        pool.execute("DELETE FROM passwords WHERE id = ?1", params![self.id])?;
        Ok(())
    }

    pub(crate) fn find_by(id: &str) -> Result<Self, FailureKind> {
        let pool = database::get()?.get()?;
        let mut statement = pool.prepare("SELECT * FROM passwords WHERE id = ?1")?;
        statement
            .query_row(params![id], |row| {
                Ok(Password {
                    id: row.get(0)?,
                    url: row.get(1)?,
                    password: row.get(2)?,
                    user_id: row.get(3)?,
                })
            })
            .map_err(|err| err.into())
    }

    pub(crate) fn get_all(user: &User) -> Result<Vec<Self>, FailureKind> {
        let pool = database::get()?.get()?;
        let mut statement = pool.prepare("SELECT * FROM passwords WHERE user_id = ?1")?;
        let mut rows = statement.query(params![user.username])?;
        let mut passwords = vec![];
        while let Some(row) = rows.next()? {
            passwords.push(Password {
                id: row.get(0)?,
                url: row.get(1)?,
                password: row.get(2)?,
                user_id: row.get(3)?,
            });
        }
        Ok(passwords)
    }
}

impl Model for Password {
    fn save(&self) -> Result<Self, FailureKind> {
        let pool = database::get()?.get()?;
        pool.execute("DELETE FROM passwords WHERE id = ?1", params![self.id])?;
        pool.execute(
            "INSERT INTO passwords(id, url, password, user_id) VALUES (?1, ?2, ?3, ?4)",
            params![self.id, self.url, self.password, self.user_id],
        )?;
        Ok(self.clone())
    }
}
