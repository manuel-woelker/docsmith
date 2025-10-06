use docsmith_base::result::DocsmithResult;
use docsmith_export_html::html_exporter::HtmlExporter;
use docsmith_model::book::Book;
use docsmith_model::chapter::Chapter;
use docsmith_model::element::Element;
use docsmith_model::value::Value;
use docsmith_pal::{FilePath, Pal, PalBox};
use docsmith_parser_markdown::summary::{SummaryEntry, parse_summary};
use std::io::{Read, Write};
use std::rc::Rc;

pub struct Transformer {
    pal: PalBox,
    exporter: HtmlExporter,
}

impl Transformer {
    pub fn new(pal: impl Pal) -> Self {
        Self {
            pal: Rc::new(pal),
            exporter: HtmlExporter::new(),
        }
    }

    pub fn transform_book(
        &self,
        summary_path: impl Into<FilePath>,
        output_path: impl Into<FilePath>,
    ) -> DocsmithResult<()> {
        let mut summary_md_content = String::new();
        self.pal
            .read_file(&summary_path.into())?
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
        self.write_html_postamble(&mut output_file)?;
        Ok(())
    }

    #[allow(clippy::only_used_in_recursion)]
    fn transform_chapter(&self, entry: &SummaryEntry) -> DocsmithResult<Chapter> {
        let mut chapter = Chapter::new(entry.label().to_string(), entry.label().clone());
        // TODO: read body from file
        //        let mut chapter_file = self.pal.read_file(&entry.path())?;
        //      chapter.body = self.pal.read_file(&entry.path())?;
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
        writeln!(output_file, "</a>",)?;
        self.exporter
            .export_value_to_html(output_file, &chapter.label)?;
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

    fn write_html_preamble(&self, output_file: &mut dyn Write, book: &Book) -> DocsmithResult<()> {
        writeln!(
            output_file,
            r#"<!DOCTYPE html>
<html>
    <head>
    <title>{}</title>
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
