use std::process::Command;

use super::super::{BuildOutput, Error};

#[inline]
pub fn compile(build_output: &mut BuildOutput) -> Result<Option<Command>, Error> {
    let out_file_path = build_output.create_out_file()?;

    let mut command = Command::new("g++");

    command
        .arg(&build_output.inputs[0])
        .arg("-o")
        .arg(&out_file_path);

    Ok(Some(command))
}
