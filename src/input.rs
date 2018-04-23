use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use tempfile::{tempdir, TempDir};

use super::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct InputFile {
    pub name: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Input {
    pub language: String,
    pub files: Vec<InputFile>,
    pub argv: Vec<String>,
}

impl Input {
    #[inline]
    pub fn create_tmp_files(&self) -> Result<(TempDir, Vec<PathBuf>), Error> {
        let dir = tempdir()?;
        let mut files = Vec::new();

        for file in &self.files {
            let tmp_file_path = dir.path().join(&file.name);

            let mut tmp_file = match File::create(&tmp_file_path) {
                Ok(file) => file,
                Err(error) => return Err(error.into()),
            };

            match tmp_file.write_all(file.content.as_bytes()) {
                Ok(_) => (),
                Err(error) => return Err(error.into()),
            }

            files.push(tmp_file_path);
        }

        Ok((dir, files))
    }
}
