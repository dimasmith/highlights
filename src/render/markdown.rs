//! Markdown format rendering for book highlights.
use crate::highlights::Book;
use crate::render::Render;
use std::io::Write;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::highlights::{Highlight, Location};

    #[test]
    fn render_book_title() {
        let new_book = Book::new("title", "author", []);

        let markdown = render_markdown(&new_book);

        assert!(markdown.contains("# title"));
        assert!(markdown.contains("*by author*"));
    }

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
}
