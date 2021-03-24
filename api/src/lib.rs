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
#[derive(Debug, Clone)]
pub struct Parameters {
    method_name: *const c_char,
    param1: *const c_char,
    param2: *const c_char,
}

/// # Safety
///
/// The function needs to have three parameters pass to him when called
#[no_mangle]
pub unsafe extern "C" fn entrypoint(params: *const Parameters) -> *mut c_void {
    INITIALIZE_LOGGER.call_once(|| simple_logger::SimpleLogger::new().init().unwrap());
    CStr::from_ptr((*params).method_name)
        .to_str()
        .map(|method_name| {
            log::info!("Entrypoint entering with {}", method_name);
            let param1 = CStr::from_ptr((*params).param1)
                .to_str()
                .map_or("", |param| param);
            let param2 = CStr::from_ptr((*params).param2)
                .to_str()
                .map_or("", |param| param);

            match method_name {
                "signin" => to_ptr(|| signin(param1, param2)),
                "signout" => to_ptr(signout),
                "createUser" => to_ptr(|| create_user(param1, param2)),
                "deleteUser" => to_ptr(|| delete_user(param1)),
                "createPassword" => to_ptr(|| create_password(param1, param2)),
                "deletePassword" => to_ptr(|| delete_password(param1)),
                "getPasswords" => to_ptr(get_passwords),
                "decrypt" => to_ptr(|| decrypt_password(param1)),
                _ => to_ptr(|| json!(FailureKind::UnknownEntrypoint)),
            }
        })
        .unwrap_or_else(|_| to_ptr(|| json!(FailureKind::Unknown("Parsing error".to_owned()))))
}

fn to_ptr<F>(call: F) -> *mut c_void
where
    F: Fn() -> serde_json::Value,
{
    serde_json::to_string(&call())
        .map(|value| CString::new(value).unwrap())
        .map(|value| value.into_raw() as *mut c_void)
        .unwrap_or_else(|_| std::ptr::null_mut())
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
        .unwrap_or_else(|_| json!(false))
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
        .unwrap_or_else(|| json!(false))
}

fn create_password(url: &str, password: &str) -> serde_json::Value {
    get_user_by_session()
        .map(|user| (user.clone(), encrypt::encrypt(&user, password)))
        .map(|(ref user, password)| {
            json!(Password::new(user, url, password.as_str()).save().is_ok())
        })
        .unwrap_or_else(|| json!(false))
}

fn delete_password(id: &str) -> serde_json::Value {
    match get_user_by_session() {
        Some(user) => Password::find_by(id)
            .map(|password| json!(user.username == password.user_id && password.destroy().is_ok()))
            .unwrap_or_else(|_| json!(false)),
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

fn decrypt_password(password: &str) -> serde_json::Value {
    match get_user_by_session() {
        Some(ref user) => match encrypt::decrypt(user, password) {
            Ok(password) => json!(password),
            Err(err) => json!(err),
        },
        None => json!(FailureKind::NotAuthorized),
    }
}

#[cfg(test)]
mod lib_test {
    use super::{entrypoint, Parameters};
    use crate::database;
    use crate::encrypt;
    use crate::model::User;
    use serial_test::serial;
    use std::{ffi::CString, os::raw::c_char};

    #[test]
    #[serial]
    fn signin() {
        database::empty();
        // Create user
        let method_name = CString::new("createUser").unwrap();
        let param1 = CString::new("test").unwrap();
        let param2 = CString::new("password").unwrap();

        let mut params = Parameters {
            method_name: method_name.as_ptr() as *const c_char,
            param1: param1.as_ptr() as *const c_char,
            param2: param2.as_ptr() as *const c_char,
        };
        unsafe {
            let result = entrypoint(Box::into_raw(Box::new(params.clone())) as *const Parameters);
            CString::from_raw(result as *mut c_char)
        };

        // Signin
        let method_name = CString::new("signin").unwrap();

        // with empty param
        let param1 = CString::new("").unwrap();
        let param2 = CString::new("").unwrap();
        params.method_name = method_name.as_ptr() as *const c_char;
        params.param1 = param1.as_ptr() as *const c_char;
        params.param2 = param2.as_ptr() as *const c_char;
        let result = unsafe {
            let result = entrypoint(Box::into_raw(Box::new(params.clone())) as *const Parameters);
            CString::from_raw(result as *mut c_char)
        };
        assert_eq!(
            false,
            serde_json::from_str::<bool>(result.to_str().unwrap()).unwrap()
        );

        // user don't exist
        let param1 = CString::new("test2").unwrap();
        let param2 = CString::new("password").unwrap();
        params.param1 = param1.as_ptr() as *const c_char;
        params.param2 = param2.as_ptr() as *const c_char;
        let result = unsafe {
            let result = entrypoint(Box::into_raw(Box::new(params.clone())) as *const Parameters);
            CString::from_raw(result as *mut c_char)
        };
        assert_eq!(
            false,
            serde_json::from_str::<bool>(result.to_str().unwrap()).unwrap()
        );

        // with wrong password
        let param1 = CString::new("test").unwrap();
        let param2 = CString::new("test").unwrap();
        params.param1 = param1.as_ptr() as *const c_char;
        params.param2 = param2.as_ptr() as *const c_char;
        let result = unsafe {
            let result = entrypoint(Box::into_raw(Box::new(params.clone())) as *const Parameters);
            CString::from_raw(result as *mut c_char)
        };
        assert_eq!(
            false,
            serde_json::from_str::<bool>(result.to_str().unwrap()).unwrap()
        );

        // Signin with true credential
        let param1 = CString::new("test").unwrap();
        let param2 = CString::new("password").unwrap();
        params.param1 = param1.as_ptr() as *const c_char;
        params.param2 = param2.as_ptr() as *const c_char;
        let result = unsafe {
            let result = entrypoint(Box::into_raw(Box::new(params.clone())) as *const Parameters);
            CString::from_raw(result as *mut c_char)
        };
        assert!(serde_json::from_str::<bool>(result.to_str().unwrap()).unwrap());
    }

    #[test]
    #[serial]
    fn create_account() {
        database::empty();
        let method_name = CString::new("createUser").unwrap();
        let param1 = CString::new("test").unwrap();
        let param2 = CString::new("password").unwrap();

        let params = Box::new(Parameters {
            method_name: method_name.as_ptr() as *const c_char,
            param1: param1.as_ptr() as *const c_char,
            param2: param2.as_ptr() as *const c_char,
        });
        let result = unsafe {
            let result = entrypoint(Box::into_raw(params) as *const Parameters);
            CString::from_raw(result as *mut c_char)
        };
        let result: User = serde_json::from_str(result.to_str().unwrap()).unwrap();
        assert_eq!("test", result.username.as_str());
        assert_eq!(encrypt::hash("password"), result.password);
    }
}
