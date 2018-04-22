use std::process;

use super::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Output {
    pub stdout: String,
    pub stderr: String,
    pub error: Option<Error>,
}

impl From<process::Output> for Output {
    #[inline]
    fn from(output: process::Output) -> Self {
        Output {
            error: if output.status.success() {
                None
            } else {
                Some((&output).into())
            },
            stdout: String::from_utf8(output.stdout).unwrap(),
            stderr: String::from_utf8(output.stderr).unwrap(),
        }
    }
}

impl From<Error> for Output {
    #[inline]
    fn from(error: Error) -> Self {
        Output {
            stdout: String::new(),
            stderr: String::new(),
            error: Some(error),
        }
    }
}

impl From<String> for Output {
    #[inline]
    fn from(error: String) -> Self {
        Self::from(Error::IO(error))
    }
}
