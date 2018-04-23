use std::path::PathBuf;
use std::process::Command;

use tempfile::TempDir;

use super::super::{create_out_file, Result};

#[inline]
pub fn run(dir: &TempDir, files: &[PathBuf], argv: &[String]) -> Result {
    let out_file_path = create_out_file(dir, "a.out")?;

    match Command::new("gcc")
        .arg(&files[0])
        .arg("-o")
        .arg(&out_file_path)
        .output()
    {
        Ok(output) => if output.status.success() {
            Ok(try_io!(Command::new(out_file_path).args(argv).output()))
        } else {
            Ok(output.into())
        },
        Err(error) => Err(error.into()),
    }
}
