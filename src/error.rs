use error_stack::ResultExt;
use std::{fmt::Display, marker::Send};

use error_stack::Report;

#[derive(Debug)]
pub enum Error {
    Message(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Message(message) => write!(f, "Error: {message}"),
        }
    }
}
impl std::error::Error for Error {}

pub trait Grab {
    #[track_caller]
    fn grab<T, E: std::error::Error + Sized + Send + Sync + 'static>(
        result: Result<T, E>,
        message: impl ToString,
    ) -> crate::Result<T> {
        result
            .map_err(Report::from)
            .change_context_lazy(|| Error::Message(message.to_string()))
    }
}
