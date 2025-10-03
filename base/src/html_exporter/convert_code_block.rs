use crate::html_exporter::convert_tag::{ConversionContext, ConvertTag};
use crate::result::DocsmithResult;
use std::io::Write;

#[derive(Default)]
pub struct ConvertCodeBlock {}

impl ConvertCodeBlock {
    pub fn new() -> Self {
        Self {}
    }
}

impl ConvertTag for ConvertCodeBlock {
    fn emit_before<'a>(
        &self,
        write: &mut dyn Write,
        _context: &ConversionContext<'a>,
    ) -> DocsmithResult<()> {
        write!(write, "<pre><code>")?;
        Ok(())
    }

    fn emit_after<'a>(
        &self,
        write: &mut dyn Write,
        _context: &ConversionContext<'a>,
    ) -> DocsmithResult<()> {
        write!(write, "</code></pre>")?;
        Ok(())
    }
}
