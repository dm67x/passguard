extern crate r2d2;
extern crate r2d2_sqlite;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate magic_crypt;
extern crate hex;
extern crate sha2;

use model::{Model, User};
use std::sync::Mutex;

mod database;
mod encrypt;
mod error;
mod model;

struct Session {
    id: Option<String>,
}

impl Default for Session {
    fn default() -> Self {
        Self { id: None }
    }
}

lazy_static! {
    static ref CURRENT_SESSION: Mutex<Session> = Mutex::new(Session::default());
}

fn set_session(id: Option<String>) {
    if let Ok(ref mut session) = CURRENT_SESSION.lock() {
        session.id = id;
    }
}

fn get_session() -> Option<String> {
    if let Ok(session) = CURRENT_SESSION.lock() {
        return session.id.clone();
    }
    None
}

pub fn create_user(name: &str, password: &str) -> bool {
    set_session(None);
    encrypt::hash(password)
        .map(|password| User::new(name, password.as_str()))
        .map(|user| user.save())
        .and_then(|user| user)
        .map(|user| set_session(Some(user.id)))
        .map(|_| true)
        .unwrap_or(false)
}

pub fn delete_user(name: &str) -> bool {
    get_session()
        .map(|session| User::find_by(session.as_str()))
        .and_then(|user| match user {
            Ok(user) => {
                if user.name == name && user.destroy().is_ok() {
                    return Some(true);
                }
                None
            }
            Err(_) => None,
        })
        .map(|_| true)
        .unwrap_or(false)
}
