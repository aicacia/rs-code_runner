use std::convert::TryInto;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;

use tempfile::{tempdir, TempDir};

use super::{create_executable, BuildInput, Error, Lang};

#[derive(Debug)]
pub struct BuildOutput {
    pub lang: Lang,
    pub root_dir: TempDir,
    pub inputs: Vec<PathBuf>,
    pub outputs: Vec<PathBuf>,
}

impl BuildOutput {
    #[inline]
    pub fn new(build_input: &BuildInput) -> Result<Self, Error> {
        let lang = build_input.lang.as_str().try_into()?;
        let dir = tempdir()?;
        let inputs_dir = dir.path().join("inputs");
        let outputs_dir = dir.path().join("outputs");
        let mut inputs = Vec::new();

        create_dir_all(outputs_dir)?;

        for (file, content) in &build_input.files {
            let tmp_file_path = inputs_dir.join(file);

            if let Some(tmp_file_path_dir) = tmp_file_path.parent() {
                create_dir_all(tmp_file_path_dir)?;
            }

            let mut tmp_file = match File::create(&tmp_file_path) {
                Ok(file) => file,
                Err(error) => return Err(error.into()),
            };

            match tmp_file.write_all(content.as_bytes()) {
                Ok(_) => (),
                Err(error) => return Err(error.into()),
            }

            inputs.push(tmp_file_path);
        }

        Ok(BuildOutput {
            lang: lang,
            root_dir: dir,
            inputs: inputs,
            outputs: Vec::new(),
        })
    }

    #[inline]
    pub fn create_out_file(&mut self) -> Result<PathBuf, Error> {
        // TODO: out paths should always be unique
        let filepath = self.root_dir.path().join("outputs").join("a.out");
        let path = create_executable(&self.root_dir, filepath)?;
        self.outputs.push(path.clone());
        Ok(path)
    }
}
