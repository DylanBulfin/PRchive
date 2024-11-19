use std::{fmt::{Display, Write}, io};

#[derive(Debug, Clone, Copy)]
pub enum ErrorKind {
    RawError,
    UreqError,
    IoError,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RawError => f.write_str("RawError"),
            Self::UreqError => f.write_str("UreqError"),
            Self::IoError => f.write_str("IoError"),
        }
    }
}

#[derive(Debug)]
pub struct Error {
    message: String,
    kind: ErrorKind,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}: {}", self.kind, self.message))
    }
}

impl From<String> for Error {
    fn from(message: String) -> Self {
        Self {
            message,
            kind: ErrorKind::RawError,
        }
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}

impl From<ureq::Error> for Error {
    fn from(value: ureq::Error) -> Self {
        Self {
            message: value.to_string(),
            kind: ErrorKind::UreqError,
        }
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self {
            message: value.to_string(),
            kind: ErrorKind::IoError,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
