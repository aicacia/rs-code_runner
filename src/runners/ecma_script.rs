use std::process::Command;

use super::super::{BuildOutput, Error, Input};

#[inline]
pub fn run(build_output: &BuildOutput, input: &Input) -> Result<Command, Error> {
    let mut command = Command::new("node");
    command.arg(&build_output.inputs[0]).args(&input.argv);
    Ok(command)
}
