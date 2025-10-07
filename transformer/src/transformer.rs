use docsmith_base::error::err;
use docsmith_base::result::DocsmithResult;
use docsmith_export_html::convert_document::PICO_CSS;
use docsmith_export_html::html_exporter::HtmlExporter;
use docsmith_model::book::Book;
use docsmith_model::chapter::Chapter;
use docsmith_model::element::Element;
use docsmith_model::tags;
use docsmith_model::value::Value;
use docsmith_pal::{FilePath, Pal, PalBox};
use docsmith_parser_markdown::markdown::parse_markdown;
use docsmith_parser_markdown::summary::{SummaryEntry, parse_summary};
use std::io::{Read, Write};
use std::rc::Rc;

pub struct Transformer {
    pal: PalBox,
    exporter: HtmlExporter,
    parent_path: FilePath,
}

impl Transformer {
    pub fn new(pal: impl Pal) -> Self {
        Self {
            pal: Rc::new(pal),
            exporter: HtmlExporter::new(),
            parent_path: FilePath::from("."),
        }
    }

    pub fn transform_book(
        &mut self,
        summary_path: impl Into<FilePath>,
        output_path: impl Into<FilePath>,
    ) -> DocsmithResult<()> {
        let mut summary_md_content = String::new();
        let summary_path = summary_path.into();
        self.parent_path = summary_path
            .parent()
            .ok_or_else(|| err!("Could not resolve parent of {}", summary_path))?
            .to_relative_path_buf();
        self.pal
            .read_file(&summary_path)?
            .read_to_string(&mut summary_md_content)?;
        let summary = parse_summary(&summary_md_content)?;
        let mut title = Element::new_tag("strong");
        title
            .children_mut()
            .push(Value::new_string("Untitled docsmith book".to_string()));
        let mut book = Book::new(title);
        for entry in summary.entries() {
            book.chapters.push(self.transform_chapter(entry)?);
        }
        let mut output_file = self.pal.create_file(&output_path.into())?;
        self.write_html_preamble(&mut output_file, &book)?;
        self.write_toc(&mut output_file, &book)?;
        self.write_content(&mut output_file, &book)?;
        self.write_html_postamble(&mut output_file)?;
        Ok(())
    }

    #[allow(clippy::only_used_in_recursion)]
    fn transform_chapter(&self, entry: &SummaryEntry) -> DocsmithResult<Chapter> {
        let mut chapter = Chapter::new(entry.label().to_string(), entry.label().clone());
        let chapter_path = self.parent_path.join(entry.path());
        let mut chapter_file = self.pal.read_file(&chapter_path)?;
        let mut chapter_content = String::new();
        chapter_file.read_to_string(&mut chapter_content)?;
        let mut chapter_element = parse_markdown(&chapter_content)?;
        chapter_element.set_tag(tags::CHAPTER);
        chapter.body = Value::new_element(chapter_element);
        for child in entry.children() {
            chapter.sub_chapters.push(self.transform_chapter(child)?);
        }
        Ok(chapter)
    }

    fn write_toc(&self, output_file: &mut dyn Write, book: &Book) -> DocsmithResult<()> {
        writeln!(output_file, "<ul>")?;
        for chapter in &book.chapters {
            self.write_toc_entry(output_file, chapter)?;
        }
        writeln!(output_file, "</ul>")?;
        Ok(())
    }

    fn write_toc_entry(
        &self,
        output_file: &mut dyn Write,
        chapter: &Chapter,
    ) -> DocsmithResult<()> {
        writeln!(output_file, "<li>")?;
        writeln!(output_file, "<a href=\"#{}\">", chapter.id,)?;
        self.exporter
            .export_value_to_html(output_file, &chapter.label)?;
        writeln!(output_file, "</a>",)?;
        if !chapter.sub_chapters.is_empty() {
            writeln!(output_file, "<ul>")?;
            for sub_chapter in &chapter.sub_chapters {
                self.write_toc_entry(output_file, sub_chapter)?;
            }
            writeln!(output_file, "</ul>")?;
        }
        writeln!(output_file, "</li>")?;
        Ok(())
    }

    fn write_content(&self, output_file: &mut dyn Write, book: &Book) -> DocsmithResult<()> {
        writeln!(output_file, "<main>")?;
        for chapter in &book.chapters {
            self.write_chapter(output_file, chapter, 1)?;
        }
        writeln!(output_file, "</main>")?;
        Ok(())
    }

    fn write_chapter(
        &self,
        output_file: &mut dyn Write,
        chapter: &Chapter,
        level: usize,
    ) -> DocsmithResult<()> {
        writeln!(output_file, "<section>")?;
        writeln!(output_file, "<a id=\"{}\"><h{level}>", chapter.id)?;
        self.exporter
            .export_value_to_html(output_file, &chapter.label)?;
        writeln!(output_file, "<h{level}></a>",)?;
        self.exporter
            .export_value_to_html(output_file, &chapter.body)?;
        if !chapter.sub_chapters.is_empty() {
            for sub_chapter in &chapter.sub_chapters {
                self.write_chapter(output_file, sub_chapter, level + 1)?;
            }
        }
        writeln!(output_file, "</section>")?;
        Ok(())
    }

    fn write_html_preamble(&self, output_file: &mut dyn Write, book: &Book) -> DocsmithResult<()> {
        writeln!(
            output_file,
            r#"<!DOCTYPE html>
<html>
    <head>
    <title>{}</title>
    <style>
    {PICO_CSS}
    </style>
    </head>
    <body>
    "#,
            book.title
        )?;
        Ok(())
    }

    fn write_html_postamble(&self, output_file: &mut dyn Write) -> DocsmithResult<()> {
        writeln!(
            output_file,
            r#"
    </body>
    </html>
    "#
        )?;
        Ok(())
    }
}
