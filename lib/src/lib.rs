use error::FailureKind;
use lazy_static::lazy_static;
use model::{Model, Password, User};
use serde_json::json;
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

fn get_user_by_session() -> Option<User> {
    get_session()
        .map(|session| User::find_by(session.as_str()))
        .map(|user| user.ok())
        .unwrap_or(None)
}

pub fn signin(name: &str, password: &str) -> serde_json::Value {
    set_session(None);
    User::find_by(name)
        .map(|user| {
            if user.password == encrypt::hash(password) {
                set_session(Some(user.username.to_string()));
                json!(true)
            } else {
                json!(false)
            }
        })
        .unwrap_or(json!(false))
}

pub fn create_user(username: &str, password: &str) -> serde_json::Value {
    User::new(username, encrypt::hash(password).as_str())
        .save()
        .map_err(|err| json!(err))
        .map(|ref user| {
            set_session(Some(user.username.clone()));
            json!(user.clone())
        })
        .unwrap_or(json!({}))
}

pub fn delete_user(username: &str) -> serde_json::Value {
    get_user_by_session()
        .map(|user| json!(user.username == username && user.destroy().is_ok()))
        .unwrap_or(json!(false))
}

pub fn create_password(url: &str, password: &str) -> serde_json::Value {
    get_user_by_session()
        .map(|user| (user.clone(), encrypt::encrypt(&user, password)))
        .map(|(ref user, password)| {
            json!(Password::new(user, url, password.as_str()).save().is_ok())
        })
        .unwrap_or(json!(false))
}

pub fn delete_password(id: &str) -> serde_json::Value {
    match get_user_by_session() {
        Some(user) => Password::find_by(id)
            .map(|password| json!(user.username == password.user_id && password.destroy().is_ok()))
            .unwrap_or(json!(false)),
        None => json!(false),
    }
}

pub fn get_passwords() -> serde_json::Value {
    match get_user_by_session() {
        Some(user) => Password::get_all(&user)
            .map(|passwords| json!(passwords))
            .unwrap_or(json!(Vec::<Password>::new())),
        None => json!(Vec::<Password>::new()),
    }
}

pub fn decrypt_password(id: &str) -> serde_json::Value {
    match get_user_by_session() {
        Some(user) => Password::find_by(id)
            .map(|password| json!(encrypt::decrypt(&user, password.password.as_str())))
            .unwrap_or(json!(FailureKind::NotAuthorized)),
        None => json!(FailureKind::NotAuthorized),
    }
}
