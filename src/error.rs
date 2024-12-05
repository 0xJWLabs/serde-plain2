use serde::de;
use serde::ser;
use std::fmt;
use thiserror::Error;

/// Errors created from this crate.
#[derive(Debug, Clone, Error)]
pub enum Error {
    /// An impossible / unsupported operation was attempted.
    #[error("cannot serialize non primitive type {0}")]
    ImpossibleSerialization(&'static str),

    /// A certain deserialization is impossible.
    #[error("cannot deserialize to non primitive type {0}")]
    ImpossibleDeserialization(&'static str),

    /// Raised when parsing errors happen during deserialization.
    #[error("cannot parse {0}: {1}")]
    Parse(&'static str, String),

    /// An arbitrary error message.
    #[error("{0}")]
    Message(String),
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error::Message(msg.to_string())
    }
}
