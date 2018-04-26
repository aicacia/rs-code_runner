use std::path::PathBuf;
use std::process::Command;

use tempfile::TempDir;

use super::super::Result;

#[inline]
pub fn run(dir: &TempDir, files: &[PathBuf], argv: &[String]) -> Result {
    let base_name = &files[0];
    let class_name = files[0].file_stem().unwrap();

    match Command::new("javac")
        .current_dir(dir.path())
        .arg(base_name)
        .output()
    {
        Ok(output) => if output.status.success() {
            Ok(try_io!(
                Command::new("java")
                    .current_dir(dir.path())
                    .arg(class_name)
                    .args(argv)
                    .output()
            ))
        } else {
            Ok(output.into())
        },
        Err(error) => Err(error.into()),
    }
}
