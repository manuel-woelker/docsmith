use crate::model::key::Key;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Element {
    tag: Key,
    attributes: HashMap<Key, String>,
    children: Vec<Element>,
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

    pub fn children(&self) -> &[Element] {
        &self.children
    }

    pub fn attributes(&self) -> &HashMap<Key, String> {
        &self.attributes
    }
}
