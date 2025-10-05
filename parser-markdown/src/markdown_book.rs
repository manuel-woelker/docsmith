use docsmith_base::result::DocsmithResult;
use docsmith_model::element::Element;
use docsmith_model::tags::BOOK;

pub fn parse_markdown_book(_input: &str) -> DocsmithResult<Element> {
    let book = BOOK.new_element();
    Ok(book)
}
