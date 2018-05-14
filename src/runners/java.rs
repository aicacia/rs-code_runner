use std::process::Command;

use super::super::{BuildOutput, Error, Input};

#[inline]
pub fn run(build_output: &BuildOutput, input: &Input) -> Result<Command, Error> {
    let class_name = build_output.inputs[0].file_stem().unwrap();

    let mut command = Command::new("java");

    command
        .current_dir(build_output.root_dir.path().join("outputs"))
        .arg(class_name)
        .args(&input.argv);

    Ok(command)
}
