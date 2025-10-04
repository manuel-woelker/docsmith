use crate::convert_tag::{ConversionContext, ConvertTag};
use docsmith_base::result::DocsmithResult;
use std::io::Write;

#[derive(Default)]
pub struct ConvertPassthru {}

impl ConvertPassthru {
    pub fn new() -> Self {
        Self {}
    }
}

impl ConvertTag for ConvertPassthru {
    fn emit_before(
        &self,
        _write: &mut dyn Write,
        _context: &ConversionContext,
    ) -> DocsmithResult<()> {
        Ok(())
    }

    fn emit_after(
        &self,
        _write: &mut dyn Write,
        _context: &ConversionContext,
    ) -> DocsmithResult<()> {
        Ok(())
    }
}
