use docsmith_base::logging::init_logging;
use docsmith_base::markdown::parse_markdown;
use docsmith_base::model::exporter::element_to_json;
use std::fs::read_to_string;
use tracing::info;

fn main() {
    init_logging();
    info!("docsmith");
    let file_content =
        read_to_string("../kubernetes-website/content/en/docs/tutorials/_index.md").unwrap();
    let root_element = parse_markdown(&file_content).unwrap();
    dbg!(&root_element);
    let json = element_to_json(&root_element).unwrap();
    println!("{}", json);
}
