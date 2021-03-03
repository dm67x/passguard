use crate::encrypt::{encrypt, hash};
use crate::error::FailureKind;
use crate::model::{Password, User};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallParams {
    method_name: String,
    user: Option<User>,
    password: Option<Password>,
}

pub fn call_api(params: CallParams) -> Result<Value, Value> {
    match params.method_name.as_str() {
        "newUser" => serialize_result(|| {
            params
                .user
                .as_ref()
                .map(|user| {
                    let user = create_new_user(user.name.as_str(), user.password.as_str())?;
                    Ok(serde_json::json!(user))
                })
                .unwrap_or(Err(FailureKind::InvalidData(
                    "Parameters are not specified".to_string(),
                )))
        }),
        "deleteUser" => serialize_result(|| {
            params
                .user
                .as_ref()
                .map(|user| {
                    delete_user(user.id.as_str())?;
                    Ok(serde_json::json!(true))
                })
                .unwrap_or(Err(FailureKind::InvalidData(
                    "Parameters are not specified".to_string(),
                )))
        }),
        "newPassword" => serialize_result(|| Ok(serde_json::json!(false))),
        "deletePassword" => serialize_result(|| Ok(serde_json::json!(false))),
        _ => serialize_result(|| Err(FailureKind::NotRecognizedEntryPoint)),
    }
}

fn serialize_result<F>(cb: F) -> Result<Value, Value>
where
    F: Fn() -> Result<Value, FailureKind>,
{
    cb().map_err(|err| serde_json::json!(err))
}

fn create_new_user(name: &str, password: &str) -> Result<User, FailureKind> {
    User::new(name, hash(password)?.as_str()).save()
}

fn delete_user(id: &str) -> Result<(), FailureKind> {
    User::find_by(id)?.destroy()
}

fn create_new_password(
    user: &User,
    website: &str,
    password: &str,
) -> Result<Password, FailureKind> {
    Password::new(website, encrypt(user, password)?.as_str()).save()
}

fn delete_password(id: &str) -> Result<(), FailureKind> {
    Password::find_by(id)?.destroy()
}
