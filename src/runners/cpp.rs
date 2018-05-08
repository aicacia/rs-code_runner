use std::process::Command;

use super::super::{BuildOutput, Error, Output};

#[inline]
pub fn run(build_output: &BuildOutput, argv: &[String]) -> Result<Output, Error> {
    Ok(try_io!(
        Command::new(&build_output.outputs[0]).args(argv).output()
    ))
}
