#![feature(try_from)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tempfile;
extern crate tokio;

#[macro_use]
mod error;

mod build_input;
mod build_output;
pub mod compilers;
mod create_executable;
mod input;
mod lang;
mod output;
mod repl;
mod run_command;
pub mod runners;

pub use self::error::Error;

pub use self::build_input::BuildInput;
pub use self::build_output::BuildOutput;
pub use self::compilers::compile;
pub use self::create_executable::create_executable;
pub use self::input::Input;
pub use self::lang::Lang;
pub use self::output::Output;
pub use self::repl::repl_start;
pub use self::run_command::run_command;
pub use self::runners::run;
