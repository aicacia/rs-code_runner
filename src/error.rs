use std::io;
use std::process::{ExitStatus, Output};

#[derive(Serialize, Deserialize, Debug)]
pub enum Error {
    Terminated,
    Code(i32),
    NotSupported(String),
    IO(String),
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
