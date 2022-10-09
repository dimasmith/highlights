//! Markdown format rendering for book highlights.
use std::io::Write;

use crate::error::HighlightError;
use crate::highlights::{Book, Highlight};
use crate::render::Render;

/// Rendering settings for markdown highlight documents.
#[derive(Debug, Clone)]
pub struct RenderSettings {
    split_lines_enabled: bool,
}

impl RenderSettings {
    pub fn new() -> Self {
        RenderSettings {
            split_lines_enabled: true,
        }
    }
    /// Split highlights via horizontal lines.
    pub fn enable_split_lines(&mut self) -> &mut RenderSettings {
        self.split_lines_enabled = true;
        self
    }

    /// Split highlights via line feeds.
    pub fn disable_split_lines(&mut self) -> &mut RenderSettings {
        self.split_lines_enabled = false;
        self
    }

    pub fn build(&self) -> Self {
        self.clone()
    }
}

/// Renders book highlights to markdown format.
pub struct MarkdownRenderer {
    render_settings: RenderSettings,
}

impl MarkdownRenderer {
    /// Create renderer with defined render settings.
    pub fn new(render_settings: RenderSettings) -> Self {
        MarkdownRenderer { render_settings }
    }

    fn do_render_book(&self, book: &Book, w: impl Write) -> std::io::Result<()> {
        let mut md = MarkdownWriter::new(w);
        md.heading(book.title())?;
        md.lf()?;
        let authors = format_args!("by {}", book.authors()).to_string();
        md.italic(&authors)?;
        md.lf()?;

        let highlights = book.highlights();
        for highlight in highlights {
            md.lf()?;
            if self.render_settings.split_lines_enabled {
                md.line()?;
            } else {
                md.lf()?;
            }
            match &highlight {
                Highlight::Quote {
                    quote: quote_text,
                    location: _,
                } => {
                    md.blockquote(quote_text)?;
                }
                Highlight::Note {
                    note: note_text,
                    location: _,
                } => {
                    md.text(note_text)?;
                }
                Highlight::Comment {
                    quote: quote_text,
                    note: note_text,
                    location: _,
                } => {
                    md.blockquote(quote_text)?;
                    md.lf()?;
                    md.text(note_text)?;
                }
            }

            md.lf()?;
            let location = highlight.location();
            let name = format_args!("Location {}", location.value()).to_string();
            md.link(&name, location.link())?;
        }

        Ok(())
    }
}

impl Render for MarkdownRenderer {
    /// Renders highlights to markdown format.
    ///
    /// ## Example:
    ///
    /// ```
    /// # use std::io::stdout;
    /// # use highlights::highlights::examples;
    /// # use highlights::render::markdown::MarkdownRenderer;
    /// # use highlights::render::Render;
    /// let mut out = stdout();
    /// let mut book = examples::chess_book();
    /// let mut renderer = MarkdownRenderer::default();
    /// renderer.render(&mut book, &mut out).unwrap();
    /// ```
    fn render(&mut self, book: &Book, out: impl Write) -> Result<(), HighlightError> {
        self.do_render_book(book, out)
            .map_err(|e| HighlightError::io("cannot write markdown notes", e))
    }
}

impl Default for MarkdownRenderer {
    fn default() -> Self {
        MarkdownRenderer::new(RenderSettings::default())
    }
}

impl Default for RenderSettings {
    fn default() -> Self {
        RenderSettings::new()
    }
}

struct MarkdownWriter<W> {
    writer: W,
}

impl<W> MarkdownWriter<W>
where
    W: Write,
{
    fn new(writer: W) -> Self {
        MarkdownWriter { writer }
    }

    fn heading(&mut self, title: &str) -> std::io::Result<()> {
        self.writer.write_fmt(format_args!("# {}\n", title))
    }

    fn blockquote(&mut self, quote: &str) -> std::io::Result<()> {
        self.writer.write_fmt(format_args!("> {}\n", quote))
    }

    fn text(&mut self, text: &str) -> std::io::Result<()> {
        self.writer.write_fmt(format_args!("{}\n", text))
    }

    fn italic(&mut self, text: &str) -> std::io::Result<()> {
        self.writer.write_fmt(format_args!("*{}*\n", text))
    }

    fn link(&mut self, title: &str, url: &str) -> std::io::Result<()> {
        self.writer
            .write_fmt(format_args!("[{}]({})\n", title, url))
    }

    fn line(&mut self) -> std::io::Result<()> {
        self.writer.write_all("---\n".as_bytes())
    }

    fn lf(&mut self) -> std::io::Result<()> {
        self.writer.write_all("\n".as_bytes())
    }
}

#[cfg(test)]
mod tests {
    mod renderer {
        use std::io::BufWriter;

        use predicates::Predicate;

        use crate::highlights::examples::basic_attributes;
        use crate::highlights::Book;
        use crate::render::markdown::{MarkdownRenderer, RenderSettings};
        use crate::render::Render;

        #[test]
        fn split_highlights_via_horizontal_lines() {
            let markdown = render_book(RenderSettings::default(), basic_attributes());

            let split_via_lines = predicates::str::contains("---").count(3);
            assert!(split_via_lines.eval(&markdown));

            assert!(markdown.contains("> Quote_1\n"));
            assert!(markdown.contains("Note_2\n"));
            assert!(markdown.contains("> Quote_3\n"));
            assert!(markdown.contains("Note_3\n"));
        }

        #[test]
        fn split_highlights_line_feeds() {
            let settings = RenderSettings::new().disable_split_lines().build();
            let markdown = render_book(settings, basic_attributes());

            let split_via_lines = predicates::str::contains("---");
            assert!(
                !split_via_lines.eval(&markdown),
                "highlights must not be split by lines"
            );
        }

        fn render_book(render_settings: RenderSettings, book: Book) -> String {
            let mut renderer = MarkdownRenderer::new(render_settings);
            let mut output = BufWriter::new(vec![]);
            renderer.render(&book, &mut output).unwrap();
            let bytes = output.buffer();
            String::from_utf8_lossy(bytes).to_string()
        }
    }

    mod settings {
        use crate::render::markdown::RenderSettings;

        #[test]
        fn disable_split_lines() {
            let mut render_settings = RenderSettings::default();

            render_settings.disable_split_lines();

            assert!(!render_settings.split_lines_enabled);
        }

        #[test]
        fn enable_split_lines() {
            let mut render_settings = RenderSettings::default();

            render_settings.enable_split_lines();

            assert!(render_settings.split_lines_enabled);
        }
    }

    mod markdown_writer {
        use std::io::BufWriter;

        use crate::render::markdown::MarkdownWriter;

        #[test]
        fn render_heading() {
            let mut buf = BufWriter::new(Vec::new());
            let mut md = MarkdownWriter::new(&mut buf);

            md.heading("Book Title").unwrap();

            let markdown = stringify(buf);
            assert_eq!(markdown, "# Book Title\n");
        }

        #[test]
        fn render_blockquote() {
            let mut buf = BufWriter::new(Vec::new());
            let mut md = MarkdownWriter::new(&mut buf);

            md.blockquote("This is rather nice quote I want to highlight")
                .unwrap();

            let markdown = stringify(buf);
            assert_eq!(
                markdown,
                "> This is rather nice quote I want to highlight\n"
            );
        }

        #[test]
        fn render_text() {
            let mut buf = BufWriter::new(Vec::new());
            let mut md = MarkdownWriter::new(&mut buf);

            md.text("Just a plain text").unwrap();

            let markdown = stringify(buf);
            assert_eq!(markdown, "Just a plain text\n");
        }

        #[test]
        fn render_line() {
            let mut buf = BufWriter::new(Vec::new());
            let mut md = MarkdownWriter::new(&mut buf);

            md.line().unwrap();

            let markdown = stringify(buf);
            assert_eq!(markdown, "---\n");
        }

        #[test]
        fn render_lf() {
            let mut buf = BufWriter::new(Vec::new());
            let mut md = MarkdownWriter::new(&mut buf);

            md.lf().unwrap();

            let markdown = stringify(buf);
            assert_eq!(markdown, "\n");
        }

        fn stringify(buffer: BufWriter<Vec<u8>>) -> String {
            let bytes = buffer.into_inner().unwrap();
            String::from_utf8(bytes).unwrap()
        }
    }
}
