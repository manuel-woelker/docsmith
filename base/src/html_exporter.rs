use crate::model::element::Element;
use crate::model::value::Value;
use crate::result::DocsmithResult;
use std::io::Write;

pub fn export_to_html(write: &mut dyn std::io::Write, document: &Element) -> DocsmithResult<()> {
    writeln!(write, "<!DOCTYPE html>")?;
    writeln!(write, "<html>")?;
    writeln!(write, "<body>")?;
    emit_element(write, document)?;
    writeln!(write, "</body>")?;
    writeln!(write, "</html>")?;
    Ok(())
}

fn emit_element(write: &mut dyn Write, element: &Element) -> DocsmithResult<()> {
    for child in element.children() {
        emit_value(write, child)?;
    }
    Ok(())
}

fn emit_value(write: &mut dyn Write, value: &Value) -> DocsmithResult<()> {
    match value {
        Value::String(string) => {
            write!(write, "{}", string)?;
        }
        Value::Element(element) => {
            emit_element(write, element)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::html_exporter::export_to_html;
    use crate::model::element::Element;
    use crate::model::value::Value;

    #[test]
    fn test_export_empty_to_html() {
        let document = Element::new_tag("root");
        let mut buffer = Vec::new();
        export_to_html(&mut buffer, &document).unwrap();
        assert_eq!(
            str::from_utf8(&buffer).unwrap(),
            "<!DOCTYPE html>\n<html>\n<body>\n</body>\n</html>\n"
        );
    }

    #[test]
    fn test_export_simple_to_html() {
        let mut document = Element::new_tag("root");
        document
            .children_mut()
            .push(Value::String("Hello, World!".to_string()));
        let mut buffer = Vec::new();
        export_to_html(&mut buffer, &document).unwrap();
        assert_eq!(
            str::from_utf8(&buffer).unwrap(),
            "<!DOCTYPE html>\n<html>\n<body>\nHello, World!</body>\n</html>\n"
        );
    }
}
