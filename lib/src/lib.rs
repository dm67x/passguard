extern crate r2d2;
extern crate r2d2_sqlite;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate magic_crypt;
extern crate hex;
extern crate sha2;

use crate::api::CallParams;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int};

mod api;
mod database;
mod encrypt;
mod error;
mod model;

#[no_mangle]
pub unsafe extern "C" fn entrypoint(method: *const c_char) -> c_int {
    match api::call_api(CallParams {
        method_name: CStr::from_ptr(method)
            .to_str()
            .map(|name| name.to_string())
            .unwrap_or("".to_string()),
        user: None,
        password: None,
    }) {
        Ok(_) => 0i32,
        Err(_) => 1i32,
    }
}
