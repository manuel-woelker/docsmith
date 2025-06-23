use crate::model::element::Element;

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
