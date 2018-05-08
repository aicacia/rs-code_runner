use std::process::Command;

use super::super::{BuildOutput, Error, Output};

#[inline]
pub fn compile(build_output: &mut BuildOutput) -> Result<Output, Error> {
    let out_file_path = build_output.create_out_file()?;

    Ok(try_io!(
        Command::new("g++")
            .arg(&build_output.inputs[0])
            .arg("-o")
            .arg(&out_file_path)
            .output()
    ))
}
