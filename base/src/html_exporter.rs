pub mod convert_document;
pub mod convert_passthru;
pub mod convert_simple;
pub mod convert_tag;
pub mod exporter;

use crate::html_exporter::exporter::HtmlExporter;
use crate::model::element::Element;
use crate::result::DocsmithResult;
use std::io::Write;

pub fn export_to_html(write: &mut dyn Write, document: &Element) -> DocsmithResult<()> {
    let exporter = HtmlExporter::new();
    exporter.export_to_html(write, document)
}

#[cfg(test)]
mod tests {
    use crate::html_exporter::exporter::HtmlExporter;
    use crate::markdown::parse_markdown;
    use crate::model::element::Element;
    use crate::model::value::Value;
    use crate::result::DocsmithResult;
    use expect_test::{Expect, expect};
    use std::collections::HashSet;

    #[test]
    fn test_export_empty_to_html() {
        let document = Element::new_tag("document");
        let mut buffer = Vec::new();
        let exporter = HtmlExporter::new();
        exporter.export_to_html(&mut buffer, &document).unwrap();
        expect![[r#"
            <!DOCTYPE html>
            <html>
            <body>
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
        let exporter = HtmlExporter::new();
        exporter.export_to_html(&mut buffer, &document).unwrap();
        expect![[r#"
            <!DOCTYPE html>
            <html>
            <body>
            Hello, World!</body>
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
