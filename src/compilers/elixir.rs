use std::process::Command;

use super::super::{BuildOutput, Error};

#[inline]
pub fn compile(_build_output: &mut BuildOutput) -> Result<Option<Command>, Error> {
    Ok(None)
}
