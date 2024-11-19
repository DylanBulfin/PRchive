pub enum ErrorKind {
    Raw,
    OctocrabError,
}

pub struct Error {
    message: String,
    kind: ErrorKind,
}

impl From<String> for Error {
    fn from(message: String) -> Self {
        Self {
            message,
            kind: ErrorKind::Raw,
        }
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}

impl From<octocrab::Error> for Error {
    fn from(value: octocrab::Error) -> Self {
        Self {
            message: value.to_string(),
            kind: ErrorKind::OctocrabError,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
