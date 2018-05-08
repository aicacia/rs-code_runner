use std::process::Command;

use super::super::{BuildOutput, Error, Output};

#[inline]
pub fn compile(build_output: &mut BuildOutput) -> Result<Output, Error> {
    let base_name = &build_output.inputs[0];

    Ok(try_io!(
        Command::new("javac")
            .current_dir(build_output.root_dir.path())
            .arg(base_name)
            .arg("-d")
            .arg("outputs")
            .output()
    ))
}
