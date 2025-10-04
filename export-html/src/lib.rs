pub mod convert_code_block;
pub mod convert_document;
pub mod convert_heading;
pub mod convert_link;
pub mod convert_passthru;
pub mod convert_simple;
pub mod convert_tag;
pub mod html_exporter;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
