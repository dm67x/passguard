use error::FailureKind;
use lazy_static::lazy_static;
use model::{Model, Password, User};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
    sync::{Mutex, Once},
};

mod database;
mod encrypt;
mod error;
mod model;

lazy_static! {
    static ref CURRENT_SESSION: Mutex<Session> = Mutex::new(Session::default());
    static ref INITIALIZE_LOGGER: Once = Once::new();
}

trait IfNone {
    type Output;
    fn if_none(&self, msg: &str) -> Result<&Self::Output, FailureKind>;
}

impl<T> IfNone for Option<T> {
    type Output = T;

    fn if_none(&self, msg: &str) -> Result<&Self::Output, FailureKind> {
        match self {
            Some(value) => Ok(value),
            None => Err(FailureKind::InvalidData(msg.to_string())),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parameters {
    method: String,
    params: Vec<String>,
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn entrypoint(params: *const c_char) -> *mut c_char {
    let params = {
        let params = CStr::from_ptr(params).to_str().unwrap();
        serde_json::from_str::<Parameters>(params).unwrap()
    };
    let result = match _entrypoint(params) {
        Ok(result) => result,
        Err(err) => json!({ "error": true, "message": err }),
    };
    CString::new(result.to_string()).unwrap().into_raw()
}

fn _entrypoint(params: Parameters) -> Result<serde_json::Value, FailureKind> {
    INITIALIZE_LOGGER.call_once(|| simple_logger::SimpleLogger::new().init().unwrap());
    let method = params.method;
    let params = params.params;
    log::info!("Entering with {}", method);
    match method.as_str() {
        "signin" => {
            let username = params.get(0).if_none("Username required")?.as_str();
            let password = params.get(1).if_none("Password required")?.as_str();
            signin(username, password)
        }
        "signout" => signout(),
        "createUser" => {
            let username = params.get(0).if_none("Username required")?.as_str();
            let password = params.get(1).if_none("Password required")?.as_str();
            create_user(username, password)
        }
        "deleteUser" => delete_user(params.get(0).if_none("Username required")?.as_str()),
        "createPassword" => create_password(
            params.get(0).if_none("URL required")?.as_str(),
            params.get(1).if_none("Password required")?.as_str(),
        ),
        "deletePassword" => delete_password(params.get(0).if_none("ID required")?.as_str()),
        "getPasswords" => get_passwords(),
        "decrypt" => decrypt_password(params.get(0).if_none("Password required")?.as_str()),
        _ => Err(FailureKind::UnknownEntrypoint),
    }
}

struct Session {
    id: Option<String>,
}

impl Default for Session {
    fn default() -> Self {
        Self { id: None }
    }
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

fn signin(name: &str, password: &str) -> Result<serde_json::Value, FailureKind> {
    set_session(None);
    let user = User::find_by(name)?;
    if user.password == encrypt::hash(password)? {
        set_session(Some(user.username.clone()));
        Ok(json!({ "username": user.username }))
    } else {
        Err(FailureKind::NotAuthorized)
    }
}

fn signout() -> Result<serde_json::Value, FailureKind> {
    match get_user_by_session() {
        Some(_) => {
            set_session(None);
            Ok(json!(true))
        }
        None => Err(FailureKind::NotAuthorized),
    }
}

fn create_user(username: &str, password: &str) -> Result<serde_json::Value, FailureKind> {
    let user = User::new(username, encrypt::hash(password)?.as_str()).save()?;
    set_session(Some(user.username.clone()));
    Ok(json!({ "username": user.username }))
}

fn delete_user(username: &str) -> Result<serde_json::Value, FailureKind> {
    match get_user_by_session() {
        Some(user) => Ok(json!(user.username == username && user.destroy().is_ok())),
        None => Err(FailureKind::NotAuthorized),
    }
}

fn create_password(url: &str, password: &str) -> Result<serde_json::Value, FailureKind> {
    match get_user_by_session() {
        Some(ref user) => {
            let encrypted_password = encrypt::encrypt(user, password)?;
            if Password::new(user, url, encrypted_password.as_str())
                .save()
                .is_err()
            {
                Err(FailureKind::InvalidData(
                    "Cannot create the password".to_string(),
                ))
            } else {
                Ok(json!(true))
            }
        }
        None => Err(FailureKind::NotAuthorized),
    }
}

fn delete_password(id: &str) -> Result<serde_json::Value, FailureKind> {
    match get_user_by_session() {
        Some(user) => {
            let password = Password::find_by(id)?;
            Ok(json!(
                user.username == password.user_id && password.destroy().is_ok()
            ))
        }
        None => Err(FailureKind::NotAuthorized),
    }
}

fn get_passwords() -> Result<serde_json::Value, FailureKind> {
    match get_user_by_session() {
        Some(user) => Ok(json!(Password::get_all(&user)?)),
        None => Err(FailureKind::NotAuthorized),
    }
}

fn decrypt_password(password: &str) -> Result<serde_json::Value, FailureKind> {
    match get_user_by_session() {
        Some(ref user) => Ok(json!(encrypt::decrypt(user, password)?)),
        None => Err(FailureKind::NotAuthorized),
    }
}
