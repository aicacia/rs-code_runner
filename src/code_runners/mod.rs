pub mod c;
pub mod cpp;
pub mod ecma_script;
pub mod elixir;
pub mod golang;
pub mod java;
pub mod lua;
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

    match input.language.as_str() {
        "c" | "gcc" | "clang" => c::run(&dir, &files, &input.argv),
        "cpp" | "c++" | "g++" | "gxx" | "clang++" => cpp::run(&dir, &files, &input.argv),
        "javascript" | "node" | "ecmascript" => ecma_script::run(&dir, &files, &input.argv),
        "elixir" => elixir::run(&dir, &files, &input.argv),
        "golang" | "go" => golang::run(&dir, &files, &input.argv),
        "java" => java::run(&dir, &files, &input.argv),
        "lua" => lua::run(&dir, &files, &input.argv),
        "python" => python::run(&dir, &files, &input.argv),
        "ruby" => ruby::run(&dir, &files, &input.argv),
        "rust" => rust::run(&dir, &files, &input.argv),
        _ => Err(Error::NotSupported(input.language.clone())),
    }
}
