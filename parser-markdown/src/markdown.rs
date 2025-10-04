use docsmith_base::error::bail;
use docsmith_base::result::DocsmithResult;
use docsmith_model::element::Element;
use docsmith_model::key::Key;
use docsmith_model::value::Value;
use pulldown_cmark::{CodeBlockKind, Event, MetadataBlockKind, Options, Tag};

pub fn parse_markdown(markdown: &str) -> DocsmithResult<Element> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    options.insert(Options::ENABLE_GFM);
    options.insert(Options::ENABLE_DEFINITION_LIST);
    options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);
    let parser = pulldown_cmark::Parser::new_ext(markdown, options);
    let root = Element::new_tag("root");
    let mut stack = vec![root];
    for (event, range) in parser.into_offset_iter() {
        match event {
            Event::Text(text) => {
                stack
                    .last_mut()
                    .expect("Top of stack is empty")
                    .children_mut()
                    .push(Value::String(text.into_string()));
            }
            Event::Start(tag) => {
                let mut element = Element::new();
                element.span_mut().start = range.start;
                element.span_mut().end = range.end;
                let tag_name = match tag {
                    Tag::Paragraph => "paragraph",
                    Tag::Heading { level, .. } => {
                        element.set_attribute("level", format!("{}", level as usize));
                        "heading"
                    }
                    Tag::Strong => "strong",
                    Tag::Emphasis => "emphasis",
                    Tag::HtmlBlock => "html_block",
                    Tag::Link {
                        link_type: _,
                        dest_url,
                        title,
                        id,
                    } => {
                        element.set_attribute("href", dest_url.to_string());
                        element.set_attribute("title", title.to_string());
                        element.set_attribute("id", id.to_string());
                        "link"
                    }
                    Tag::List(_firstitemnumber) => "list",
                    Tag::Item => "item",
                    Tag::MetadataBlock(metadata) => {
                        match metadata {
                            MetadataBlockKind::YamlStyle => { /* supported */ }
                            _ => {
                                todo!("Unsupported metadata style: {:?}", metadata);
                            }
                        }
                        "metadata"
                    }
                    Tag::CodeBlock(CodeBlockKind::Indented) => "code_block",
                    Tag::CodeBlock(CodeBlockKind::Fenced(language)) => {
                        element.set_attribute("language", language.into_string());
                        "code_block"
                    }
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
            Event::Rule => {
                dbg!(event);
            }
            Event::SoftBreak => {
                dbg!(event);
            }
            Event::Html(html) => {
                dbg!(html);
            }
            Event::Code(code) => {
                let mut element = Element::new();
                element.span_mut().start = range.start;
                element.span_mut().end = range.end;
                element.set_tag("code");
                element
                    .children_mut()
                    .push(Value::String(code.into_string()));
                stack
                    .last_mut()
                    .expect("Top of stack is empty")
                    .children_mut()
                    .push(Value::Element(element));
            }
            other => todo!("Implement {:?}", other),
        }
    }
    let mut root = stack.pop().expect("stack empty");
    let metadata_key = &Key::from("metadata");
    // convert metadata
    root.walk_mut(|element| {
        if element.tag() == metadata_key {
            let mut metadata_string = String::new();
            let children = std::mem::take(element.children_mut());
            for child in children {
                if let Value::String(str) = child {
                    metadata_string.push_str(&str);
                } else {
                    bail!("Found non-string child in metadata block: {:?}", child)
                }
            }
            let mut key_maybe = None;
            let parser = saphyr_parser::Parser::new_from_str(&metadata_string);
            let mut mapping_depth = 0;
            for event in parser {
                let (event, _span) = event?;
                use saphyr_parser::Event;
                match event {
                    Event::Nothing => {}
                    Event::StreamStart => {}
                    Event::StreamEnd => {}
                    Event::DocumentStart(_) => {}
                    Event::DocumentEnd => {}
                    Event::Alias(_) => {}
                    Event::Scalar(value, _, _, _) => {
                        if mapping_depth != 1 {
                            bail!(
                                "Unexpected mapping depth in YAML frontmatter: {:?}",
                                metadata_string
                            );
                        }
                        if let Some(key) = key_maybe {
                            element.set_attribute(key, Value::String(value.to_string()));
                            key_maybe = None;
                        } else {
                            key_maybe = Some(Key::from(value.to_string()));
                        }
                    }
                    Event::SequenceStart(_, _) => {}
                    Event::SequenceEnd => {}
                    Event::MappingStart(_, _) => {
                        mapping_depth += 1;
                    }
                    Event::MappingEnd => {
                        mapping_depth -= 1;
                    }
                }
            }
        }
        Ok(())
    })?;
    Ok(root)
}

#[cfg(test)]
mod tests {
    use super::parse_markdown;
    use docsmith_base::result::DocsmithResult;
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
                root (0+0)
            "#]),
        )
    }

    #[test]
    fn test_parse_plain() -> DocsmithResult<()> {
        test_parse(
            "foobar",
            expect!([r#"
                root (0+0)
                  paragraph (0+6)
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
                root (0+0)
                  heading (0+6)
                    @level: "1"
                    "one"
                  heading (7+7)
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
                root (0+0)
                  paragraph (0+15)
                    "foo "
                    strong (4+7)
                      "bar"
                    " baz"
            "#]),
        )
    }

    #[test]
    fn test_parse_mixed() -> DocsmithResult<()> {
        test_parse(
            "**foo bar _fizz buzz_**",
            expect!([r#"
                root (0+0)
                  paragraph (0+23)
                    strong (0+23)
                      "foo bar "
                      emphasis (10+11)
                        "fizz buzz"
            "#]),
        )
    }

    #[test]
    fn test_parse_metadata() -> DocsmithResult<()> {
        test_parse(
            r#"
---
foo: bar
fizz: buzz
---
## two

"#,
            expect!([r#"
                root (0+0)
                  metadata (1+27)
                    @fizz: "buzz"
                    @foo: "bar"
                  heading (29+7)
                    @level: "2"
                    "two"
            "#]),
        )
    }
}
