use crate::model::element::Element;
use pulldown_cmark::CowStr;

#[derive(Debug)]
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

impl<'a> From<CowStr<'a>> for Value {
    fn from(value: CowStr<'a>) -> Self {
        value.to_string().into()
    }
}

impl Value {
    pub fn as_string(&self) -> &str {
        match self {
            Value::String(string) => string,
            Value::Element(element) => panic!("Element {} is not a string", element.tag()),
        }
    }
}
