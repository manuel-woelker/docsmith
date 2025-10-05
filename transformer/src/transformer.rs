use docsmith_base::result::DocsmithResult;
use docsmith_pal::{FilePath, Pal, PalBox};
use docsmith_parser_markdown::summary::{Summary, SummaryEntry, parse_summary};
use std::io::{Read, Write};
use std::rc::Rc;

pub struct Transformer {
    pal: PalBox,
}

impl Transformer {
    pub fn new(pal: impl Pal) -> Self {
        Self { pal: Rc::new(pal) }
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
        let mut output_file = self.pal.create_file(&output_path.into())?;
        self.write_html_preamble(&mut output_file)?;
        self.write_toc(&mut output_file, &summary)?;
        self.write_html_postamble(&mut output_file)?;
        Ok(())
    }

    fn write_toc(&self, output_file: &mut dyn Write, summary: &Summary) -> DocsmithResult<()> {
        writeln!(output_file, "<ul>")?;
        for entry in summary.entries() {
            Self::write_toc_entry(output_file, entry)?;
        }
        writeln!(output_file, "</ul>")?;
        Ok(())
    }

    fn write_toc_entry(output_file: &mut dyn Write, entry: &SummaryEntry) -> DocsmithResult<()> {
        writeln!(output_file, "<li>")?;
        writeln!(
            output_file,
            "<a href=\"{}\">{}</a>",
            entry.path(),
            entry.title()
        )?;
        if !entry.children().is_empty() {
            writeln!(output_file, "<ul>")?;
            for child in entry.children() {
                Self::write_toc_entry(output_file, child)?;
            }
            writeln!(output_file, "</ul>")?;
        }
        writeln!(output_file, "</li>")?;
        Ok(())
    }

    fn write_html_preamble(&self, output_file: &mut dyn Write) -> DocsmithResult<()> {
        writeln!(
            output_file,
            r#"<!DOCTYPE html>
<html>
    <head>
    <title>Docsmith Book</title>
    </head>
    <body>
    "#
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
