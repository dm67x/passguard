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
        if self.username.is_empty() || self.password.is_empty() {
            return Err(FailureKind::InvalidData(
                "Username or password is empty".to_owned(),
            ));
        }

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
        if self.url.is_empty() || self.password.is_empty() {
            return Err(FailureKind::InvalidData(
                "URL or password is empty".to_owned(),
            ));
        }

        let pool = database::get()?.get()?;
        pool.execute("DELETE FROM passwords WHERE id = ?1", params![self.id])?;
        pool.execute(
            "INSERT INTO passwords(id, url, password, user_id) VALUES (?1, ?2, ?3, ?4)",
            params![self.id, self.url, self.password, self.user_id],
        )?;
        Ok(self.clone())
    }
}

#[cfg(test)]
mod model_test {
    use super::{Model, Password, User};
    use crate::database;
    use rusqlite::params;

    fn get_users() -> Vec<User> {
        let pool = database::get().unwrap().get().unwrap();
        let mut statement = pool
            .prepare("SELECT username, password FROM users")
            .unwrap();
        let mut rows = statement.query(params![]).unwrap();
        let mut users = vec![];
        while let Some(row) = rows.next().unwrap() {
            users.push(User {
                username: row.get(0).unwrap(),
                password: row.get(1).unwrap(),
            });
        }
        users
    }

    fn get_passwords() -> Vec<Password> {
        let pool = database::get().unwrap().get().unwrap();
        let mut statement = pool
            .prepare("SELECT id, url, password, user_id FROM passwords")
            .unwrap();
        let mut rows = statement.query(params![]).unwrap();
        let mut passwords = vec![];
        while let Some(row) = rows.next().unwrap() {
            passwords.push(Password {
                id: row.get(0).unwrap(),
                url: row.get(1).unwrap(),
                password: row.get(2).unwrap(),
                user_id: row.get(3).unwrap(),
            });
        }
        passwords
    }

    #[test]
    fn test_user_save() {
        database::empty();
        assert!(get_users().is_empty());
        assert!(User::new("", "").save().is_err());
        User::new("test", "test").save().unwrap();
        User::new("test", "test").save().unwrap();
        assert_eq!(1, get_users().len());
    }

    #[test]
    fn test_user_destroy() {
        database::empty();
        let user = User::new("test", "test").save().unwrap();
        assert_eq!(1, get_users().len());
        user.destroy().unwrap();
        assert!(get_users().is_empty());
    }

    #[test]
    fn test_user_find_by() {
        database::empty();
        User::new("test", "test").save().unwrap();
        User::new("test2", "test").save().unwrap();
        assert!(User::find_by("test").is_ok());
        assert!(User::find_by("test2").is_ok());
        assert_ne!(
            "test2",
            User::find_by("test")
                .map(|user| user.username)
                .unwrap_or("".to_owned())
                .as_str()
        );
    }

    #[test]
    fn test_password_save() {
        database::empty();
        assert!(get_passwords().is_empty());
        let user = User::new("test", "test").save().unwrap();
        assert!(Password::new(&user, "", "").save().is_err());
        Password::new(&user, "test", "test").save().unwrap();
        assert_eq!(1, get_passwords().len());
        Password::new(&user, "test", "test").save().unwrap();
        assert_eq!(2, get_passwords().len());
    }

    #[test]
    fn test_password_destroy() {
        database::empty();
        let user = User::new("test", "test").save().unwrap();
        let password = Password::new(&user, "test", "test").save().unwrap();
        assert_eq!(1, get_passwords().len());
        password.destroy().unwrap();
        assert!(get_passwords().is_empty());
    }

    #[test]
    fn test_password_find_by_id() {
        database::empty();
        let user = User::new("test", "test").save().unwrap();
        let password = Password::new(&user, "test", "test").save().unwrap();
        assert!(Password::find_by("").is_err());
        assert!(Password::find_by(password.id.as_str()).is_ok());
    }

    #[test]
    fn test_password_get_all() {
        database::empty();
        let user = User::new("test", "test").save().unwrap();
        let user2 = User::new("test2", "test").save().unwrap();
        assert!(Password::get_all(&user).unwrap().is_empty());
        assert!(Password::get_all(&user2).unwrap().is_empty());
        Password::new(&user, "test", "test").save().unwrap();
        assert_eq!(1, Password::get_all(&user).unwrap().len());
        assert!(Password::get_all(&user2).unwrap().is_empty());
        Password::new(&user2, "test", "test").save().unwrap();
        assert_eq!(1, Password::get_all(&user).unwrap().len());
        assert_eq!(1, Password::get_all(&user2).unwrap().len());
    }
}
