use std::fs::OpenOptions;
use std::path::PathBuf;

use tempfile::TempDir;

use super::Error;

#[inline]
pub fn create_out_file(dir: &TempDir, name: &str) -> Result<PathBuf, Error> {
    let out_file_path = dir.path().join(name);

    {
        let mut options = OpenOptions::new();

        options.create(true).write(true);

        if cfg!(unix) {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o755);
        }

        match options.open(&out_file_path) {
            Ok(_) => (),
            Err(error) => return Err(error.into()),
        }
    }

    Ok(out_file_path)
}
