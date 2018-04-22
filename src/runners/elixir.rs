use std::path::PathBuf;
use std::process::Command;

use tempfile::TempDir;

use super::super::Result;

#[inline]
pub fn run(_dir: &TempDir, files: &[PathBuf], stdin: &[&str]) -> Result {
    Ok(try_io!(
        Command::new("elixir").arg(&files[0]).args(stdin).output()
    ))
}