use crate::model::element::Element;
use crate::result::DocsmithResult;
use std::io::Write;

pub struct ConversionContext<'a> {
    pub element: &'a Element,
}

impl<'a> ConversionContext<'a> {
    pub fn new(element: &'a Element) -> Self {
        Self { element }
    }
}

pub trait ConvertTag {
    fn emit_before<'a>(
        &self,
        write: &mut dyn Write,
        context: &ConversionContext<'a>,
    ) -> DocsmithResult<()>;
    fn emit_after<'a>(
        &self,
        write: &mut dyn Write,
        context: &ConversionContext<'a>,
    ) -> DocsmithResult<()>;
}
