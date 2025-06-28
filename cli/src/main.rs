use docsmith_base::logging::init_logging;
use docsmith_base::markdown::parse_markdown;
use std::fs::read_to_string;
use tracing::info;

fn main() {
    init_logging();
    info!("docsmith");
    let file_content =
        read_to_string("../kubernetes-website/content/en/docs/tutorials/_index.md").unwrap();
    let result = parse_markdown(&file_content).unwrap();
    dbg!(result);
}
