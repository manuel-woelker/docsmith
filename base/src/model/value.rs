use crate::model::element::Element;

#[derive(Debug, PartialEq, Eq)]
pub enum Value {
    String(String),
    Element(Element),
}

impl Value {
    pub fn new_string(value: String) -> Value {
        Value::String(value)
    }

    pub fn new_element(value: Element) -> Value {
        Value::Element(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Value {
        Value::String(value)
    }
}
