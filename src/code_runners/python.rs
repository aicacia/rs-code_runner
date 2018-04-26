use std::path::PathBuf;
use std::process::Command;

use tempfile::TempDir;

use super::super::Result;

#[inline]
pub fn run(_dir: &TempDir, files: &[PathBuf], argv: &[String]) -> Result {
    Ok(try_io!(
        Command::new("python").arg(&files[0]).args(argv).output()
    ))
}
