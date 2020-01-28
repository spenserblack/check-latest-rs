use std::fmt::{self, Display};

/// Result of version checking/comparison.
pub type Result<T> = std::result::Result<T, Error>;

/// Error for failed version checking/comparison.
#[derive(Debug)]
pub struct Error {
    message: String,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error {
            message: s.into(),
        }
    }
}
