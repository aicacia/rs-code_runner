use std::process::Command;

use super::super::{BuildOutput, Error, Output};

#[inline]
pub fn run(build_output: &BuildOutput, argv: &[String]) -> Result<Output, Error> {
    Ok(try_io!(
        Command::new("elixir")
            .arg(&build_output.inputs[0])
            .args(argv)
            .output()
    ))
}
