use super::{Error, Output};

pub type Result = ::std::result::Result<Output, Error>;

#[macro_export]
macro_rules! try_io {
    ($expr:expr) => {
        match $expr {
            Ok(value) => value.into(),
            Err(error) => return Err(error.into()),
        }
    };
}
