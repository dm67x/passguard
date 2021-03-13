use crate::error::FailureKind;
use crate::model::User;
use magic_crypt::MagicCryptTrait;
use sha2::{Digest, Sha256};

pub(crate) fn encrypt(user: &User, password: &str) -> Result<String, FailureKind> {
    let mc = new_magic_crypt!(&user.id.as_str(), 256);
    Ok(mc.encrypt_bytes_to_base64(password))
}

pub(crate) fn decrypt(user: &User, password: &str) -> Result<String, FailureKind> {
    let mc = new_magic_crypt!(&user.id.as_str(), 256);
    Ok(mc.decrypt_base64_to_string(password)?)
}

pub(crate) fn hash(password: &str) -> Result<String, FailureKind> {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    Ok(hex::encode(hasher.finalize()))
}
