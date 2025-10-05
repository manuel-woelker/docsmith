use docsmith_base::logging::init_logging;
use docsmith_model::tags;
use docsmith_parser_markdown::markdown::parse_markdown;
use std::fs::read_to_string;
use std::time::Instant;
use tracing::info;

fn main() {
    init_logging();
    info!("docsmith");
    let start = Instant::now();
    let file_content = read_to_string("sample-documents/rust-rfc-0069-ascii-literals.md").unwrap();
    let mut root_element = parse_markdown(&file_content).unwrap();
    root_element.set_tag(tags::ARTICLE);
    //dbg!(&root_element);
    //let json = element_to_json(&root_element).unwrap();
    // println!("{}", json);
    let mut output_file = std::fs::File::create("target/output.html").unwrap();
    let exporter = docsmith_export_html::html_exporter::HtmlExporter::new();
    exporter
        .export_to_html(&mut output_file, &root_element)
        .unwrap();
    for unhandled_tag in exporter.unhandled_tags() {
        println!("Unhandled tag: {}", unhandled_tag);
    }
    let duration = start.elapsed();
    println!("Duration: {} ms", duration.as_millis());
}
