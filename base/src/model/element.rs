use crate::model::key::Key;
use crate::model::value::Value;
use std::collections::HashMap;

#[derive(PartialEq, Eq)]
pub struct Element {
    tag: Key,
    attributes: HashMap<Key, Value>,
    children: Vec<Value>,
}

impl Element {
    pub fn new(tag: impl Into<Key>) -> Element {
        Element {
            tag: tag.into(),
            attributes: HashMap::new(),
            children: Vec::new(),
        }
    }

    pub fn tag(&self) -> &Key {
        &self.tag
    }

    pub fn set_tag(&mut self, tag: impl Into<Key>) {
        self.tag = tag.into();
    }

    pub fn children(&self) -> &[Value] {
        &self.children
    }

    pub fn children_mut(&mut self) -> &mut Vec<Value> {
        &mut self.children
    }

    pub fn attributes(&self) -> &HashMap<Key, Value> {
        &self.attributes
    }

    pub fn set_attribute(&mut self, key: impl Into<Key>, value: impl Into<Value>) {
        self.attributes.insert(key.into(), value.into());
    }
}

fn fmt_element(
    element: &Element,
    f: &mut std::fmt::Formatter<'_>,
    mut indent: usize,
) -> std::fmt::Result {
    writeln!(f, "{}", element.tag)?;

    indent += 2;

    for (key, value) in &element.attributes {
        write!(f, "{:indent$}@{}: ", "", key, indent = indent)?;
        fmt_value(value, f, indent)?;
    }

    for child in &element.children {
        write!(f, "{:indent$}", "", indent = indent)?;
        fmt_value(child, f, indent)?;
    }
    Ok(())
}

fn fmt_value(value: &Value, f: &mut std::fmt::Formatter<'_>, indent: usize) -> std::fmt::Result {
    match value {
        Value::String(string) => {
            f.write_str("\"")?;
            f.write_str(string)?;
            f.write_str("\"\n")
        }
        Value::Element(element) => fmt_element(element, f, indent),
    }?;
    Ok(())
}

impl std::fmt::Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_element(self, f, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::{Expect, expect};

    #[test]
    fn test_debug() {
        fn test_element(element: &Element, expected: Expect) {
            expected.assert_eq(&format!("{:?}", element));
        }
        let mut element = Element::new("div");
        test_element(
            &element,
            expect![[r#"
            div
        "#]],
        );

        element.children_mut().push(Value::String("foo".into()));
        test_element(
            &element,
            expect![[r#"
                div
                  "foo"
            "#]],
        );

        element.children_mut().clear();
        element
            .children_mut()
            .push(Value::Element(Element::new("link")));
        test_element(
            &element,
            expect![[r#"
                div
                  link
            "#]],
        );

        element.children_mut().clear();
        element
            .attributes
            .insert(Key::from("class"), Value::String("foo".into()));
        test_element(
            &element,
            expect![[r#"
                div
                  @class: "foo"
            "#]],
        );

        element.children_mut().clear();
        let mut inner_element = Element::new("foo");
        inner_element
            .attributes
            .insert(Key::from("href"), Value::String("bar".into()));
        inner_element.children.push(Value::String("child".into()));
        element
            .attributes
            .insert(Key::from("class"), Value::Element(inner_element));
        test_element(
            &element,
            expect![[r#"
                div
                  @class: foo
                    @href: "bar"
                    "child"
            "#]],
        );
    }
}
