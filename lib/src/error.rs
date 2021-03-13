use serde::{Deserialize, Serialize};
use std::sync::PoisonError;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum FailureKind {
    SqliteError(String),
    R2d2Error(String),
    LoggerError(String),
    PoisonError(String),
    EncryptionError(String),
    FormatError(String),
    InvalidData(String),
    IoError(String),
    NotAuthorized,
}

impl From<rusqlite::Error> for FailureKind {
    fn from(err: rusqlite::Error) -> Self {
        Self::SqliteError(err.to_string())
    }
}

impl From<r2d2::Error> for FailureKind {
    fn from(err: r2d2::Error) -> Self {
        Self::R2d2Error(err.to_string())
    }
}

impl From<log::SetLoggerError> for FailureKind {
    fn from(err: log::SetLoggerError) -> Self {
        Self::LoggerError(err.to_string())
    }
}

impl<T> From<PoisonError<T>> for FailureKind {
    fn from(err: PoisonError<T>) -> Self {
        Self::PoisonError(err.to_string())
    }
}

impl From<magic_crypt::MagicCryptError> for FailureKind {
    fn from(err: magic_crypt::MagicCryptError) -> Self {
        Self::EncryptionError(err.to_string())
    }
}

impl From<std::string::FromUtf8Error> for FailureKind {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Self::FormatError(err.to_string())
    }
}

impl From<std::io::Error> for FailureKind {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err.to_string())
    }
}
