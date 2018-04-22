extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate tempfile;

mod create_out_file;
mod error;
mod input;
mod output;
#[macro_use]
mod result;
pub mod runners;

pub use self::create_out_file::create_out_file;
pub use self::error::Error;
pub use self::input::{Input, InputFile};
pub use self::output::Output;
pub use self::result::Result;
pub use self::runners::run;
