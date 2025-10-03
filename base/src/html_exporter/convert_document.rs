use crate::html_exporter::convert_tag::{ConversionContext, ConvertTag};
use crate::result::DocsmithResult;
use std::io::Write;

#[derive(Default)]
pub struct ConvertDocument {}

impl ConvertDocument {
    pub fn new() -> Self {
        Self {}
    }
}

impl ConvertTag for ConvertDocument {
    fn emit_before<'a>(
        &self,
        write: &mut dyn Write,
        _context: &ConversionContext<'a>,
    ) -> DocsmithResult<()> {
        writeln!(write, "<!DOCTYPE html>")?;
        writeln!(write, "<html>")?;
        writeln!(write, "<body>")?;
        Ok(())
    }

    fn emit_after<'a>(
        &self,
        write: &mut dyn Write,
        _context: &ConversionContext<'a>,
    ) -> DocsmithResult<()> {
        writeln!(write, "</body>")?;
        writeln!(write, "</html>")?;
        Ok(())
    }
}
