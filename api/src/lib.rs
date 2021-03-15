use error::FailureKind;
use lazy_static::lazy_static;
use model::{Model, Password, User};
use serde_json::json;
use std::{
    ffi::{c_void, CStr, CString},
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

#[repr(C)]
#[derive(Debug)]
pub struct Parameters {
    method_name: *const c_char,
    username: *const c_char,
    password: *const c_char,
}

#[no_mangle]
pub unsafe extern "C" fn entrypoint(params: *const Parameters) -> *mut c_void {
    //INITIALIZE_LOGGER.call_once(|| simple_logger::SimpleLogger::new().init().unwrap());
    CStr::from_ptr((*params).method_name)
        .to_str()
        .map(|method_name| {
            println!("Entrypoint entering with {}", method_name);
            match method_name {
                "createUser" => {
                    let username = CStr::from_ptr((*params).username)
                        .to_str()
                        .map_or("", |name| name);
                    let password = CStr::from_ptr((*params).password)
                        .to_str()
                        .map_or("", |password| password);

                    if username.is_empty() || password.is_empty() {
                        value_to_ptr(None)
                    } else {
                        value_to_ptr(Some(create_user(username, password)))
                    }
                }
                _ => value_to_ptr(None),
            }
        })
        .unwrap_or(value_to_ptr(None))
}

fn value_to_ptr(value: Option<serde_json::Value>) -> *mut c_void {
    value
        .map(|value| {
            let value = value.as_str().map(|value| value).unwrap_or("");
            CString::new(value)
                .map(|value| value.into_raw())
                .unwrap_or(std::ptr::null_mut())
        })
        .unwrap_or(std::ptr::null_mut()) as *mut c_void
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

fn signin(name: &str, password: &str) -> serde_json::Value {
    set_session(None);
    User::find_by(name)
        .map(|user| {
            if user.password == encrypt::hash(password) {
                set_session(Some(user.username));
                json!(true)
            } else {
                json!(false)
            }
        })
        .unwrap_or(json!(false))
}

fn signout() -> serde_json::Value {
    set_session(None);
    json!(true)
}

fn create_user(username: &str, password: &str) -> serde_json::Value {
    User::new(username, encrypt::hash(password).as_str())
        .save()
        .map_err(|err| json!(err))
        .map(|ref user| {
            set_session(Some(user.username.clone()));
            json!(user.clone())
        })
        .unwrap_or_else(|_| json!({}))
}

fn delete_user(username: &str) -> serde_json::Value {
    get_user_by_session()
        .map(|user| json!(user.username == username && user.destroy().is_ok()))
        .unwrap_or(json!(false))
}

fn create_password(url: &str, password: &str) -> serde_json::Value {
    get_user_by_session()
        .map(|user| (user.clone(), encrypt::encrypt(&user, password)))
        .map(|(ref user, password)| {
            json!(Password::new(user, url, password.as_str()).save().is_ok())
        })
        .unwrap_or(json!(false))
}

fn delete_password(id: &str) -> serde_json::Value {
    match get_user_by_session() {
        Some(user) => Password::find_by(id)
            .map(|password| json!(user.username == password.user_id && password.destroy().is_ok()))
            .unwrap_or(json!(false)),
        None => json!(false),
    }
}

fn get_passwords() -> serde_json::Value {
    match get_user_by_session() {
        Some(user) => Password::get_all(&user)
            .map(|passwords| json!(passwords))
            .unwrap_or_else(|_| json!(Vec::<Password>::new())),
        None => json!(Vec::<Password>::new()),
    }
}

fn decrypt_password(id: &str) -> serde_json::Value {
    match get_user_by_session() {
        Some(user) => Password::find_by(id)
            .map(|password| json!(encrypt::decrypt(&user, password.password.as_str())))
            .unwrap_or_else(|_| json!(FailureKind::NotAuthorized)),
        None => json!(FailureKind::NotAuthorized),
    }
}
