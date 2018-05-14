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

use super::{run_command, BuildOutput, Error, Input, Lang, Output};

#[inline]
pub fn run(build_output: &BuildOutput, input: &Input) -> Result<Output, Error> {
    match match &build_output.lang {
        &Lang::C => c::run(build_output, input),
        &Lang::Cpp => cpp::run(build_output, input),
        &Lang::EcmaScript => ecma_script::run(build_output, input),
        &Lang::Elixir => elixir::run(build_output, input),
        &Lang::Go => go::run(build_output, input),
        &Lang::Java => java::run(build_output, input),
        &Lang::Lua => lua::run(build_output, input),
        &Lang::Python => python::run(build_output, input),
        &Lang::Ruby => ruby::run(build_output, input),
        &Lang::Rust => rust::run(build_output, input),
    } {
        Ok(command) => run_command(command, input.timeout),
        Err(e) => Err(e),
    }
}
