use std::process::Command;

use super::super::{BuildOutput, Error, Output};

#[inline]
pub fn run(build_output: &BuildOutput, argv: &[String]) -> Result<Output, Error> {
    let class_name = build_output.inputs[0].file_stem().unwrap();

    Ok(try_io!(
        Command::new("java")
            .current_dir(build_output.root_dir.path().join("outputs"))
            .arg(class_name)
            .args(argv)
            .output()
    ))
}
