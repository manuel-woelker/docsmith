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
    fn emit_before<'a>(
        &self,
        _write: &mut dyn Write,
        _context: &ConversionContext<'a>,
    ) -> DocsmithResult<()> {
        Ok(())
    }

    fn emit_after<'a>(
        &self,
        _write: &mut dyn Write,
        _context: &ConversionContext<'a>,
    ) -> DocsmithResult<()> {
        Ok(())
    }
}
