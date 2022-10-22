//! Markdown format rendering for book highlights.
use std::io::Write;

use crate::error::HighlightError;
use crate::highlights::{Book, Highlight};
use crate::render::markdown::writer::MarkdownWriter;
use crate::render::Render;

mod writer;

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
pub fn render_book(book: &Book, w: impl Write) -> std::io::Result<()> {
    let mut md = MarkdownWriter::new(w);
    md.heading(book.title())?.lf()?;
    let authors = format_args!("by {}", book.authors()).to_string();
    md.italic(&authors)?.lf()?;

    let highlights = book.highlights();
    for highlight in highlights {
        md.lf()?.line()?;
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
    fn render(&mut self, book: &Book, out: impl Write) -> Result<(), HighlightError> {
        render_book(book, out).map_err(|e| HighlightError::io("cannot write markdown notes", e))
    }
}

impl Default for MarkdownRenderer {
    fn default() -> Self {
        MarkdownRenderer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::highlights::Location;

    #[test]
    fn render_title() {
        let book = Book::new("Title", "Author", []);

        let markdown = render_markdown(&book);
        let lines: Vec<&str> = markdown.lines().collect();

        assert!(lines.contains(&"# Title"));
        assert!(lines.contains(&"*by Author*"));
    }

    #[test]
    fn render_quote() {
        let book = Book::new(
            "Title",
            "Author",
            [Highlight::quote(
                "Quote",
                Location::new(1, "http://book.org/quotes/1"),
            )],
        );

        let markdown = render_markdown(&book);
        let lines: Vec<&str> = markdown.lines().collect();

        assert!(lines.contains(&"> Quote"));
        assert!(lines.contains(&"[Location 1](http://book.org/quotes/1)"));
    }

    #[test]
    fn render_note() {
        let book = Book::new(
            "Title",
            "Author",
            [Highlight::note(
                "Note",
                Location::new(1, "http://book.org/notes/1"),
            )],
        );

        let markdown = render_markdown(&book);
        let lines: Vec<&str> = markdown.lines().collect();

        assert!(lines.contains(&"Note"));
        assert!(lines.contains(&"[Location 1](http://book.org/notes/1)"));
    }

    #[test]
    fn render_comment() {
        let book = Book::new(
            "Title",
            "Author",
            [Highlight::comment(
                "Quote",
                "Note",
                Location::new(1, "http://book.org/comments/1"),
            )],
        );

        let markdown = render_markdown(&book);
        let lines: Vec<&str> = markdown.lines().collect();

        assert!(lines.contains(&"> Quote"));
        assert!(lines.contains(&"Note"));
        assert!(lines.contains(&"[Location 1](http://book.org/comments/1)"));
    }

    fn render_markdown(new_book: &Book) -> String {
        let mut renderer = MarkdownRenderer::default();
        renderer.as_string(new_book)
    }
}
