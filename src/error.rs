use std::fmt::Display;

#[derive(Debug)]
pub enum FailureKind {
    DatabaseError(rusqlite::Error),
}

impl Display for FailureKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DatabaseError(err) => write!(f, "Database error: {}", err),
        }
    }
}

impl From<rusqlite::Error> for FailureKind {
    fn from(err: rusqlite::Error) -> Self {
        FailureKind::DatabaseError(err)
    }
}
