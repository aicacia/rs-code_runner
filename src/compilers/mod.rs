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

use super::{run_command, BuildOutput, Error, Lang, Output};

#[inline]
pub fn compile(build_output: &mut BuildOutput, timeout: f32) -> Result<Output, Error> {
    match match &build_output.lang {
        &Lang::C => c::compile(build_output),
        &Lang::Cpp => cpp::compile(build_output),
        &Lang::EcmaScript => ecma_script::compile(build_output),
        &Lang::Elixir => elixir::compile(build_output),
        &Lang::Go => go::compile(build_output),
        &Lang::Java => java::compile(build_output),
        &Lang::Lua => lua::compile(build_output),
        &Lang::Python => python::compile(build_output),
        &Lang::Ruby => ruby::compile(build_output),
        &Lang::Rust => rust::compile(build_output),
    } {
        Ok(Some(command)) => run_command(command, timeout),
        Ok(None) => Ok(Output::default()),
        Err(e) => Err(e),
    }
}
