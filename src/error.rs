use std::{fmt::Display, sync::PoisonError};

#[derive(Debug)]
pub enum FailureKind {
    RusqliteError(rusqlite::Error),
    R2d2Error(r2d2::Error),
    LoggerError(log::SetLoggerError),
    PoisonError(String),
}

impl Display for FailureKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RusqliteError(err) => write!(f, "RusqliteError: {}", err),
            Self::R2d2Error(err) => write!(f, "R2d2Error error: {}", err),
            Self::LoggerError(err) => write!(f, "Logger error: {}", err),
            Self::PoisonError(err) => write!(f, "Poison Error: {}", err),
        }
    }
}

impl From<rusqlite::Error> for FailureKind {
    fn from(err: rusqlite::Error) -> Self {
        Self::RusqliteError(err)
    }
}

impl From<r2d2::Error> for FailureKind {
    fn from(err: r2d2::Error) -> Self {
        Self::R2d2Error(err)
    }
}

impl From<log::SetLoggerError> for FailureKind {
    fn from(err: log::SetLoggerError) -> Self {
        Self::LoggerError(err)
    }
}

impl<T> From<PoisonError<T>> for FailureKind {
    fn from(err: PoisonError<T>) -> Self {
        Self::PoisonError(err.to_string())
    }
}
