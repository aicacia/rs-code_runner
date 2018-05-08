use std::fs::OpenOptions;
use std::path::{Path, PathBuf};

use tempfile::TempDir;

use super::Error;

#[inline]
pub fn create_executable<P: AsRef<Path>>(dir: &TempDir, name: P) -> Result<PathBuf, Error> {
    let path = dir.path().join(name);

    {
        let mut options = OpenOptions::new();

        options.create(true).read(true).write(true);

        if cfg!(unix) {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o755);
        }

        match options.open(&path) {
            Ok(_) => (),
            Err(error) => return Err(error.into()),
        }
    }

    Ok(path)
}

#[test]
fn create_executable_test() {
    use std::process::Command;
    use tempfile::tempdir;

    let dir = tempdir().unwrap();
    let path = create_executable(&dir, "a.out").unwrap();

    let output = Command::new(&path).output().unwrap();
    assert_eq!(output.status.code().unwrap(), 0);
}
