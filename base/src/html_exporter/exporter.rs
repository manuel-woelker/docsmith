use crate::html_exporter::convert_document::ConvertDocument;
use crate::html_exporter::convert_passthru::ConvertPassthru;
use crate::html_exporter::convert_simple::ConvertSimple;
use crate::html_exporter::convert_tag::{ConversionContext, ConvertTag};
use crate::model::element::Element;
use crate::model::key::Key;
use crate::model::value::Value;
use crate::result::DocsmithResult;
use pulldown_cmark_escape::{IoWriter, escape_html_body_text};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::io::Write;

pub struct HtmlExporter {
    converter_map: HashMap<Key, Box<dyn ConvertTag>>,
    unhandled_tags: RefCell<HashSet<Key>>,
}

impl HtmlExporter {
    pub fn new() -> Self {
        let mut exporter = Self {
            converter_map: HashMap::new(),
            unhandled_tags: RefCell::new(HashSet::new()),
        };
        exporter.register_converter("root", ConvertPassthru::new());
        exporter.register_converter("document", ConvertDocument::new());
        let mut register_tag = |docsmith_tag: &'static str, html_tag: &'static str| {
            exporter.register_converter(docsmith_tag, ConvertSimple::new(html_tag));
        };
        register_tag("paragraph", "p");
        register_tag("strong", "strong");
        exporter
    }

    pub fn register_converter(
        &mut self,
        key: impl Into<Key>,
        converter: impl ConvertTag + 'static,
    ) {
        self.converter_map.insert(key.into(), Box::new(converter));
    }

    pub fn unhandled_tags(&self) -> HashSet<Key> {
        self.unhandled_tags.borrow().clone()
    }
}

impl Default for HtmlExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl HtmlExporter {
    pub fn export_to_html(&self, write: &mut dyn Write, document: &Element) -> DocsmithResult<()> {
        self.export_element(write, document)
    }

    fn export_element(&self, mut write: &mut dyn Write, element: &Element) -> DocsmithResult<()> {
        let converter = self.converter_map.get(element.tag());
        let conversion_context = ConversionContext::new(element);
        if let Some(converter) = converter {
            converter.emit_before(write, &conversion_context)?;
        } else {
            self.unhandled_tags
                .borrow_mut()
                .insert(element.tag().clone());
        }
        for child in element.children() {
            match child {
                Value::Element(element) => {
                    self.export_element(write, element)?;
                }
                Value::String(text) => {
                    let mut writer = IoWriter(&mut write);
                    escape_html_body_text(&mut writer, text)?;
                }
            }
        }
        if let Some(converter) = converter {
            converter.emit_after(write, &conversion_context)?;
        }
        Ok(())
    }
}
