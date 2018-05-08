use super::super::{BuildOutput, Error, Output};

#[inline]
pub fn compile(_build_output: &mut BuildOutput) -> Result<Output, Error> {
    Ok(Output::default())
}
