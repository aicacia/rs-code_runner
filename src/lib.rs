#![feature(try_from)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate tempfile;

#[macro_use]
mod error;

mod build_input;
mod build_output;
pub mod compilers;
mod create_executable;
mod lang;
mod output;
pub mod runners;

pub use self::error::Error;

pub use self::build_input::BuildInput;
pub use self::build_output::BuildOutput;
pub use self::compilers::compile;
pub use self::create_executable::create_executable;
pub use self::lang::Lang;
pub use self::output::Output;
pub use self::runners::run;
