use std::io;
use std::process::{ExitStatus, Output};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Error {
    Timeout,
    Terminated,
    Code(i32),
    NotSupported(String),
    IO(String),
    Other(String),
}

impl<'a> From<&'a str> for Error {
    #[inline]
    fn from(error: &'a str) -> Self {
        Error::Other(error.to_owned())
    }
}

impl From<String> for Error {
    #[inline]
    fn from(error: String) -> Self {
        Error::Other(error)
    }
}

impl From<io::Error> for Error {
    #[inline]
    fn from(error: io::Error) -> Self {
        Error::IO(error.to_string())
    }
}

impl From<Output> for Error {
    #[inline]
    fn from(output: Output) -> Self {
        Self::from(&output)
    }
}

impl<'a> From<&'a Output> for Error {
    #[inline]
    fn from(output: &'a Output) -> Self {
        Self::from(&output.status)
    }
}

impl<'a> From<&'a ExitStatus> for Error {
    #[inline]
    fn from(exit_status: &'a ExitStatus) -> Self {
        match exit_status.code() {
            Some(code) => Error::Code(code),
            None => Error::Terminated,
        }
    }
}

#[macro_export]
macro_rules! try_io {
    ($expr:expr) => {
        match $expr {
            Ok(value) => value.into(),
            Err(error) => return Err(error.into()),
        }
    };
}
