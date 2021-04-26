use crate::error::FailureKind;
use crate::model::User;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use sha2::{Digest, Sha256};

pub(crate) fn encrypt(user: &User, password: &str) -> Result<String, FailureKind> {
    if password.is_empty() {
        Err(FailureKind::InvalidData(
            "Password cannot be empty".to_string(),
        ))
    } else {
        Ok(new_magic_crypt!(&user.username.as_str(), 256).encrypt_bytes_to_base64(password))
    }
}

pub(crate) fn decrypt(user: &User, password: &str) -> Result<String, FailureKind> {
    Ok(new_magic_crypt!(&user.username.as_str(), 256).decrypt_base64_to_string(password)?)
}

pub(crate) fn hash(password: &str) -> Result<String, FailureKind> {
    if password.is_empty() {
        Err(FailureKind::InvalidData(
            "Password cannot be empty".to_string(),
        ))
    } else {
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        Ok(hex::encode(hasher.finalize()))
    }
}
