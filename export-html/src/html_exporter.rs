use crate::convert_code_block::ConvertCodeBlock;
use crate::convert_document::ConvertDocument;
use crate::convert_heading::ConvertHeading;
use crate::convert_link::ConvertLink;
use crate::convert_passthru::ConvertPassthru;
use crate::convert_simple::ConvertSimple;
use crate::convert_tag::{ConversionContext, ConvertTag};
use docsmith_base::result::DocsmithResult;
use docsmith_model::element::Element;
use docsmith_model::key::Key;
use docsmith_model::value::Value;
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
        exporter.register_converter("heading", ConvertHeading::new());
        exporter.register_converter("link", ConvertLink::new());
        exporter.register_converter("code_block", ConvertCodeBlock::new());
        let mut register_tag = |docsmith_tag: &'static str, html_tag: &'static str| {
            exporter.register_converter(docsmith_tag, ConvertSimple::new(html_tag));
        };

        register_tag("paragraph", "p");
        register_tag("strong", "strong");
        register_tag("code", "code");
        register_tag("list", "ul");
        register_tag("item", "li");
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

pub fn export_to_html(write: &mut dyn Write, document: &Element) -> DocsmithResult<()> {
    let exporter = HtmlExporter::new();
    exporter.export_to_html(write, document)
}

#[cfg(test)]
mod tests {
    use crate::convert_document::ConvertDocument;
    use crate::html_exporter::HtmlExporter;
    use docsmith_base::result::DocsmithResult;
    use docsmith_model::element::Element;
    use docsmith_model::value::Value;
    use docsmith_parser_markdown::markdown::parse_markdown;
    use expect_test::{Expect, expect};
    use std::collections::HashSet;

    #[test]
    fn test_export_empty_to_html() {
        let document = Element::new_tag("document");
        let mut buffer = Vec::new();
        let mut exporter = HtmlExporter::new();
        exporter.register_converter("document", ConvertDocument::new_inline_css("body {}"));
        exporter.export_to_html(&mut buffer, &document).unwrap();
        expect![[r#"
            <!DOCTYPE html>
            <html>
            <head>
            <style>
            body {}
            </style>
            </head>
            <body>
            <main>
            </main>
            </body>
            </html>
        "#]]
        .assert_eq(str::from_utf8(&buffer).unwrap());
        assert_eq!(exporter.unhandled_tags(), HashSet::new());
    }

    #[test]
    fn test_export_simple_to_html() {
        let mut document = Element::new_tag("document");
        document
            .children_mut()
            .push(Value::String("Hello, World!".to_string()));
        let mut buffer = Vec::new();
        let mut exporter = HtmlExporter::new();
        exporter.register_converter("document", ConvertDocument::new_inline_css("body {}"));
        exporter.export_to_html(&mut buffer, &document).unwrap();
        expect![[r#"
            <!DOCTYPE html>
            <html>
            <head>
            <style>
            body {}
            </style>
            </head>
            <body>
            <main>
            Hello, World!</main>
            </body>
            </html>
        "#]]
        .assert_eq(str::from_utf8(&buffer).unwrap());
        assert_eq!(exporter.unhandled_tags(), HashSet::new());
    }

    fn test_export(input: &str, expected: Expect) -> DocsmithResult<()> {
        let document = parse_markdown(input)?;
        let mut buffer = Vec::new();
        let exporter = HtmlExporter::new();
        exporter.export_to_html(&mut buffer, &document).unwrap();
        expected.assert_eq(str::from_utf8(&buffer).unwrap());
        assert_eq!(exporter.unhandled_tags(), HashSet::new());
        Ok(())
    }

    macro_rules! test_export {
        ($name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $name() -> DocsmithResult<()> {
                test_export($input, $expected)
            }
        };
    }

    test_export!(empty, "", expect![""]);
    test_export!(simple, "foobar", expect!["<p>foobar</p>"]);
    test_export!(
        bold,
        "**Bolded**",
        expect!["<p><strong>Bolded</strong></p>"]
    );
    test_export!(
        escapes,
        "LT: < GT: > AMP: & QUOTE: \" SINGLE QUOTE: '",
        expect![[r#"<p>LT: &lt; GT: &gt; AMP: &amp; QUOTE: " SINGLE QUOTE: '</p>"#]]
    );
}
