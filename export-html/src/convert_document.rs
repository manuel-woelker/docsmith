use crate::convert_tag::{ConversionContext, ConvertTag};
use docsmith_base::result::DocsmithResult;
use std::borrow::Cow;
use std::io::Write;

const CSS: &str = include_str!("css/pico.classless.jade.css");

#[derive(Default)]
pub struct ConvertDocument {
    css: Option<Cow<'static, str>>,
}

impl ConvertDocument {
    pub fn new() -> Self {
        Self {
            css: Some(Cow::Borrowed(CSS)),
        }
    }

    pub fn new_inline_css(css: impl Into<Cow<'static, str>>) -> Self {
        Self {
            css: Some(css.into()),
        }
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
        writeln!(write, "<head>")?;
        if let Some(css) = &self.css {
            writeln!(write, "<style>")?;
            writeln!(write, "{}", css)?;
            writeln!(write, "</style>")?;
        }
        writeln!(write, "</head>")?;
        writeln!(write, "<body>")?;
        writeln!(write, "<main>")?;
        Ok(())
    }

    fn emit_after<'a>(
        &self,
        write: &mut dyn Write,
        _context: &ConversionContext<'a>,
    ) -> DocsmithResult<()> {
        writeln!(write, "</main>")?;
        writeln!(write, "</body>")?;
        writeln!(write, "</html>")?;
        Ok(())
    }
}
