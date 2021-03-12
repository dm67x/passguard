extern crate r2d2;
extern crate r2d2_sqlite;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate magic_crypt;
extern crate hex;
extern crate sha2;

use model::{Model, User};
use std::os::raw::{c_char, c_int};
use std::{ffi::CStr, sync::Mutex};

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

#[no_mangle]
pub unsafe extern "C" fn create_user(name: *const c_char, password: *const c_char) -> c_int {
    set_session(None);
    let name = CStr::from_ptr(name)
        .to_str()
        .expect("create_user failed to cast to str");
    let password = CStr::from_ptr(password)
        .to_str()
        .expect("create_user failed to cast to str");
    encrypt::hash(password)
        .map(|password| User::new(name, password.as_str()))
        .map(|user| user.save())
        .and_then(|user| user)
        .map(|user| set_session(Some(user.id)))
        .map(|_| 0i32)
        .unwrap_or(1i32)
}

#[no_mangle]
pub unsafe extern "C" fn delete_user(name: *const c_char) -> c_int {
    let name = CStr::from_ptr(name)
        .to_str()
        .expect("delete_user failed to cast to str");
    get_session()
        .map(|session| User::find_by(session.as_str()))
        .and_then(|user| match user {
            Ok(user) => {
                if user.name == name {
                    if let Ok(_) = user.destroy() {
                        return Some(0i32);
                    }
                }
                None
            }
            Err(_) => None,
        })
        .map(|_| 0i32)
        .unwrap_or(1i32)
}
