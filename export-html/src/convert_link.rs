use crate::convert_tag::{ConversionContext, ConvertTag};
use docsmith_base::result::DocsmithResult;
use docsmith_model::value::Value;
use std::io::Write;

#[derive(Default)]
pub struct ConvertLink {}

impl ConvertLink {
    pub fn new() -> Self {
        Self {}
    }
}

impl ConvertTag for ConvertLink {
    fn emit_before<'a>(
        &self,
        write: &mut dyn Write,
        context: &ConversionContext<'a>,
    ) -> DocsmithResult<()> {
        if let Some(href) = context.element.get_attribute("href").map(Value::as_string) {
            write!(write, "<a href=\"{}\">", href)?;
        } else {
            write!(write, "<a>")?;
        }
        Ok(())
    }

    fn emit_after<'a>(
        &self,
        write: &mut dyn Write,
        _context: &ConversionContext<'a>,
    ) -> DocsmithResult<()> {
        write!(write, "</a>")?;
        Ok(())
    }
}
