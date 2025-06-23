use crate::model::element::Element;
use crate::model::value::Value;
use crate::result::DocsmithResult;
use pulldown_cmark::{Event, Tag};

pub fn parse_markdown(markdown: &str) -> DocsmithResult<Element> {
    let parser = pulldown_cmark::Parser::new(markdown);
    let root = Element::new("root");
    let mut stack = vec![root];
    for event in parser {
        match event {
            Event::Text(text) => {
                stack
                    .last_mut()
                    .expect("Top of stack is empty")
                    .children_mut()
                    .push(Value::String(text.into_string()));
            }
            Event::Start(tag) => {
                let mut element = Element::new("");
                let tag_name = match tag {
                    Tag::Paragraph => "paragraph",
                    Tag::Heading { level, .. } => {
                        element.set_attribute("level", format!("{}", level as usize));
                        "heading"
                    }
                    Tag::Strong => "strong",
                    _ => todo!("Implement tag: {:?}", tag),
                };
                element.set_tag(tag_name);
                stack.push(element);
            }
            Event::End(_tag_end) => {
                let top = stack.pop().expect("Top of stack is empty");
                stack
                    .last_mut()
                    .expect("Top of stack is empty")
                    .children_mut()
                    .push(Value::Element(top));
            }
            other => todo!("Implement {:?}", other),
        }
    }
    Ok(stack.pop().expect("stack empty"))
}

#[cfg(test)]
mod tests {
    use super::parse_markdown;
    use crate::result::DocsmithResult;
    use expect_test::{Expect, expect};

    fn test_parse(markdown: &str, expected: Expect) -> DocsmithResult<()> {
        let element = parse_markdown(markdown)?;
        expected.assert_eq(&format!("{:?}", element));
        Ok(())
    }

    #[test]
    fn test_parse_empty() -> DocsmithResult<()> {
        test_parse(
            "",
            expect!([r#"
            root
        "#]),
        )
    }

    #[test]
    fn test_parse_plain() -> DocsmithResult<()> {
        test_parse(
            "foobar",
            expect!([r#"
            root
              paragraph
                "foobar"
        "#]),
        )
    }

    #[test]
    fn test_parse_headings() -> DocsmithResult<()> {
        test_parse(
            r#"# one

## two

"#,
            expect!([r#"
    root
      heading
        @level: "1"
        "one"
      heading
        @level: "2"
        "two"
"#]),
        )
    }

    #[test]
    fn test_parse_bold() -> DocsmithResult<()> {
        test_parse(
            "foo **bar** baz",
            expect!([r#"
            root
              paragraph
                "foo "
                strong
                  "bar"
                " baz"
        "#]),
        )
    }
}
