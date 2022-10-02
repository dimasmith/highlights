//! Rendering abstraction to export highlights.
use std::io::{BufWriter, Write};

use crate::error::HighlightError;
use crate::highlights::Book;

pub mod markdown;

/// Render format to export book highlights.
pub trait Render {
    /// Render book into specified output.
    fn render(&mut self, book: &Book, out: impl Write) -> Result<(), HighlightError>;

    /// Renders book highlights into the string.
    ///
    /// This is a convenience method to simplify testing new renderers.
    fn as_string(&mut self, book: &Book) -> String {
        let mut buf = BufWriter::new(Vec::new());
        self.render(book, &mut buf).unwrap();
        let bytes = buf.into_inner().unwrap();
        String::from_utf8(bytes).unwrap()
    }
}
