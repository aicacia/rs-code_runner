use std::process::Command;

use super::super::{BuildOutput, Error, Input};

#[inline]
pub fn run(build_output: &BuildOutput, input: &Input) -> Result<Command, Error> {
    let mut command = Command::new(&build_output.outputs[0]);
    command.args(&input.argv);
    Ok(command)
}
