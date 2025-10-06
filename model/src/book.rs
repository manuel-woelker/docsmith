use crate::chapter::Chapter;
use crate::element::Element;

pub struct Book {
    pub title: Element,
    pub authors: Vec<String>,
    pub chapters: Vec<Chapter>,
}

impl Book {
    pub fn new(title: Element) -> Book {
        Book {
            title,
            authors: vec![],
            chapters: vec![],
        }
    }
}
