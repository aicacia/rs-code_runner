use std::process::Command;

use super::super::{BuildOutput, Error};

#[inline]
pub fn compile(build_output: &mut BuildOutput) -> Result<Option<Command>, Error> {
    let base_name = &build_output.inputs[0];
    let mut command = Command::new("javac");

    command
        .current_dir(build_output.root_dir.path())
        .arg(base_name)
        .arg("-d")
        .arg("outputs");

    Ok(Some(command))
}
