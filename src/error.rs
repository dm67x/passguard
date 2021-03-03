use serde::{Deserialize, Serialize};
use std::{fmt::Display, sync::PoisonError};

#[derive(Debug, Serialize, Deserialize)]
pub enum FailureKind {
    SqliteError(String),
    R2d2Error(String),
    LoggerError(String),
    PoisonError(String),
    EncryptionError(String),
    FormatError(String),
    NotRecognizedEntryPoint,
    InvalidData(String),
    IoError(String),
}

impl Display for FailureKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SqliteError(err) => write!(f, "Sqlite error: {}", err),
            Self::R2d2Error(err) => write!(f, "R2d2Error error: {}", err),
            Self::LoggerError(err) => write!(f, "Logger error: {}", err),
            Self::PoisonError(err) => write!(f, "Poison error: {}", err),
            Self::EncryptionError(err) => write!(f, "Encryption error: {}", err),
            Self::FormatError(err) => write!(f, "Format error: {}", err),
            Self::IoError(err) => write!(f, "IO Error: {}", err),
            Self::NotRecognizedEntryPoint => write!(f, "Entrypoint not recognized"),
            Self::InvalidData(err) => write!(f, "Invalid Data: {}", err),
        }
    }
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
