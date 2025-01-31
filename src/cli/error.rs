use std::{fmt::Display, num::ParseIntError, str::ParseBoolError};

#[derive(Debug)]
pub enum Error {
    Bool(ParseBoolError),
    Int(ParseIntError),
    Msg(String),
    Io(std::io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool(e) => write!(f, "error parsing bool: {}", e),
            Self::Int(e) => write!(f, "error parsing int: {}", e),
            Self::Msg(e) => write!(f, "{}", e),
            Self::Io(e) => write!(f, "io error: {}", e),
        }
    }
}

impl From<ParseBoolError> for Error {
    fn from(value: ParseBoolError) -> Self {
        Error::Bool(value)
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Error::Int(value)
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error::Msg(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Io(value)
    }
}

impl From<std::io::ErrorKind> for Error {
    fn from(value: std::io::ErrorKind) -> Self {
        Error::Io(value.into())
    }
}
