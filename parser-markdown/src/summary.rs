use docsmith_base::error::err;
use docsmith_base::result::DocsmithResult;
use indenter::indented;
use pulldown_cmark::{Event, Options, Tag, TagEnd};
use std::fmt::Write;
use std::fmt::{Debug, Formatter};

#[derive(Default, Clone)]
pub struct Summary {
    entries: Vec<SummaryEntry>,
}

impl Summary {
    pub fn new() -> Summary {
        Summary {
            entries: Vec::new(),
        }
    }
}

impl Debug for Summary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Summary:")?;
        let mut indenter = indented(f);
        for entry in &self.entries {
            write!(indenter, "{:?}", entry)?;
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct SummaryEntry {
    title: String,
    path: String,
    children: Vec<SummaryEntry>,
}

impl SummaryEntry {
    pub fn new(title: String, path: String) -> SummaryEntry {
        SummaryEntry {
            title,
            path,
            children: Vec::new(),
        }
    }
}

impl Debug for SummaryEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}: {}", self.title, self.path)?;
        let mut indenter = indented(f);
        for child in &self.children {
            write!(indenter, "{:?}", child)?;
        }
        Ok(())
    }
}

pub fn parse_summary(input: &str) -> DocsmithResult<Summary> {
    let mut summary = Summary::new();
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    options.insert(Options::ENABLE_GFM);
    options.insert(Options::ENABLE_DEFINITION_LIST);
    options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);
    let parser = pulldown_cmark::Parser::new_ext(input, options);
    let mut link_text = String::new();
    let mut stack: Vec<SummaryEntry> =
        vec![SummaryEntry::new("document".to_string(), "".to_string())];
    for event in parser {
        match event {
            Event::Start(Tag::Item) => {
                stack.push(SummaryEntry::new("".to_string(), "".to_string()));
            }
            Event::Text(text) => {
                link_text.push_str(&text);
            }
            Event::Start(Tag::Link { dest_url, .. }) => {
                let top = stack.last_mut().ok_or_else(|| err!("empty stack"))?;
                link_text.clear();
                top.path = dest_url.to_string();
            }
            Event::End(TagEnd::Link) => {
                let top = stack.last_mut().ok_or_else(|| err!("empty stack"))?;
                top.title = link_text.clone();
            }
            Event::End(TagEnd::Item) => {
                let entry = stack.pop().ok_or_else(|| err!("empty stack"))?;
                stack
                    .last_mut()
                    .ok_or_else(|| err!("empty stack"))?
                    .children
                    .push(entry);
            }
            _ => {}
        }
    }
    summary.entries = stack.pop().ok_or_else(|| err!("empty stack"))?.children;
    Ok(summary)
}

#[cfg(test)]
mod tests {
    use crate::summary::parse_summary;
    use expect_test::{Expect, expect};

    #[test]
    fn empty() {
        test_summary(
            "",
            expect![[r#"
            Summary:

        "#]],
        );
    }

    #[test]
    fn simple() {
        test_summary(
            r#"
- [Introduction](Introduction.md)
- [Table of Contents](toc.md)
        "#,
            expect![[r#"
            Summary:
                Introduction: Introduction.md
                Table of Contents: toc.md

        "#]],
        );
    }

    #[test]
    fn nested() {
        test_summary(
            r#"
 - [foo](foo.md)
   - [bar](bar.md)
   - [baz](baz.md)
 - [fizz](fizz.md)
   - [buzz](buzz.md)
     - [bizz](bizz.md)
        "#,
            expect![[r#"
            Summary:
                foo: foo.md
                    bar: bar.md
                    baz: baz.md
                fizz: fizz.md
                    buzz: buzz.md
                        bizz: bizz.md

        "#]],
        );
    }

    fn test_summary(input: &str, expect: Expect) {
        let summary = parse_summary(input).unwrap();
        expect.assert_debug_eq(&summary);
    }
}
