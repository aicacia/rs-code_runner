pub mod c;
pub mod cpp;
pub mod ecma_script;
pub mod elixir;
pub mod java;
pub mod python;
pub mod ruby;
pub mod rust;

use super::{Error, Input, Result};

#[inline]
pub fn run(input: &Input) -> Result {
    let (dir, files) = match input.create_tmp_files() {
        Ok(result) => result,
        Err(error) => return Err(error.into()),
    };

    let stdin = input.stdin.split(" ").collect::<Vec<&str>>();

    match input.language.as_str() {
        "c" => c::run(&dir, &files, &stdin),
        "cpp" | "c++" => cpp::run(&dir, &files, &stdin),
        "javascript" | "node" | "ecmascript" => ecma_script::run(&dir, &files, &stdin),
        "java" => java::run(&dir, &files, &stdin),
        "elixir" => elixir::run(&dir, &files, &stdin),
        "python" => python::run(&dir, &files, &stdin),
        "ruby" => ruby::run(&dir, &files, &stdin),
        "rust" => rust::run(&dir, &files, &stdin),
        _ => Err(Error::NotSupported(input.language.clone())),
    }
}
