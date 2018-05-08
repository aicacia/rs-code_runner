pub mod c;
pub mod cpp;
pub mod ecma_script;
pub mod elixir;
pub mod go;
pub mod java;
pub mod lua;
pub mod python;
pub mod ruby;
pub mod rust;

use super::{BuildOutput, Error, Lang, Output};

#[inline]
pub fn run(build_output: &BuildOutput, argv: &[String]) -> Result<Output, Error> {
    match &build_output.lang {
        &Lang::C => c::run(build_output, argv),
        &Lang::Cpp => cpp::run(build_output, argv),
        &Lang::EcmaScript => ecma_script::run(build_output, argv),
        &Lang::Elixir => elixir::run(build_output, argv),
        &Lang::Go => go::run(build_output, argv),
        &Lang::Java => java::run(build_output, argv),
        &Lang::Lua => lua::run(build_output, argv),
        &Lang::Python => python::run(build_output, argv),
        &Lang::Ruby => ruby::run(build_output, argv),
        &Lang::Rust => rust::run(build_output, argv),
    }
}
