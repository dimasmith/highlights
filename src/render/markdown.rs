//! Markdown format rendering for book highlights.
use std::io::Write;

use crate::highlights::Book;
use crate::render::Render;

/// Renders the book into markdown format using supplied writer.
///
/// Use renderer abstraction where possible.
///
/// ## Example
///
/// ```
/// # use std::io::stdout;
/// # use highlights::highlights::examples;
/// # use highlights::render::markdown::render_book;
/// let mut writer = stdout();
/// let mut book = examples::chess_book();
/// render_book(&mut book, &mut writer).unwrap();
/// ```
/// Produces the markdown output of the example book into the standard output.
pub fn render_book<W>(book: &Book, w: &mut W) -> std::io::Result<()>
where
    W: Write,
{
    w.write_fmt(format_args!("# {}", book.title()))?;
    w.write_all("\n\n".as_bytes())?;
    w.write_fmt(format_args!("*by {}*", book.authors()))?;
    w.write_all("\n\n".as_bytes())?;

    let highlights = book.highlights();
    for highlight in highlights {
        w.write_fmt(format_args!("> {}", highlight.text()))?;
        w.write_all("\n> \n".as_bytes())?;
        let location = highlight.location();
        w.write_fmt(format_args!(
            "> [Location {}]({})",
            location.value(),
            location.link()
        ))?;
        w.write_all("\n\n".as_bytes())?;
    }

    Ok(())
}

/// Renders book highlights to markdown format.
pub struct MarkdownRenderer;

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
    fn render<W>(&mut self, book: &Book, out: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        render_book(book, out)
    }
}

impl Default for MarkdownRenderer {
    fn default() -> Self {
        MarkdownRenderer
    }
}

#[allow(dead_code)]
struct MarkdownWriter<W> {
    writer: W,
}

#[allow(dead_code)]
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
    use std::io::BufWriter;

    use crate::highlights::{Highlight, Location};

    use super::*;

    #[test]
    fn render_highlights() {
        let new_book = Book::new(
            "other title",
            "new author",
            [
                Highlight::new("highlight 1", Location::new(1, "loc1")),
                Highlight::new("highlight 2", Location::new(2, "loc2")),
            ],
        );

        let markdown = render_markdown(&new_book);

        assert!(markdown.contains("# other title"));
        assert!(markdown.contains("*by new author*"));
        assert!(markdown.contains("> highlight 1"));
        assert!(markdown.contains("> [Location 1](loc1)"));
        assert!(markdown.contains("> highlight 2"));
        assert!(markdown.contains("> [Location 2](loc2)"));
    }

    fn render_markdown(new_book: &Book) -> String {
        let mut renderer = MarkdownRenderer::default();
        renderer.as_string(new_book)
    }

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
