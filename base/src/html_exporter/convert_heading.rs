use crate::html_exporter::convert_tag::{ConversionContext, ConvertTag};
use crate::result::DocsmithResult;
use std::io::Write;

#[derive(Default)]
pub struct ConvertHeading {}

impl ConvertHeading {
    pub fn new() -> Self {
        Self {}
    }

    fn get_level<'a>(context: &'a ConversionContext) -> &'a str {
        context
            .element
            .get_attribute("level")
            .map(|level| level.as_string())
            .unwrap_or("1")
    }
}

impl ConvertTag for ConvertHeading {
    fn emit_before<'a>(
        &self,
        write: &mut dyn Write,
        context: &ConversionContext<'a>,
    ) -> DocsmithResult<()> {
        let level = Self::get_level(context);
        write!(write, "<h{}>", level)?;
        Ok(())
    }

    fn emit_after<'a>(
        &self,
        write: &mut dyn Write,
        context: &ConversionContext<'a>,
    ) -> DocsmithResult<()> {
        let level = Self::get_level(context);
        write!(write, "</h{}>", level)?;
        Ok(())
    }
}
