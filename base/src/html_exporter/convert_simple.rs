use crate::html_exporter::convert_tag::{ConversionContext, ConvertTag};
use crate::result::DocsmithResult;
use std::borrow::Cow;
use std::io::Write;

pub struct ConvertSimple {
    html_tag: Cow<'static, str>,
}

impl ConvertSimple {
    pub fn new(html_tag: impl Into<Cow<'static, str>>) -> Self {
        Self {
            html_tag: html_tag.into(),
        }
    }
}

impl ConvertTag for ConvertSimple {
    fn emit_before<'a>(
        &self,
        write: &mut dyn Write,
        _context: &ConversionContext<'a>,
    ) -> DocsmithResult<()> {
        write!(write, "<{}>", self.html_tag)?;
        Ok(())
    }

    fn emit_after<'a>(
        &self,
        write: &mut dyn Write,
        _context: &ConversionContext<'a>,
    ) -> DocsmithResult<()> {
        write!(write, "</{}>", self.html_tag)?;
        Ok(())
    }
}
