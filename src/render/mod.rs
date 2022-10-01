//! Rendering abstraction to export highlights.
use crate::highlights::Book;
use std::io::{BufWriter, Write};

pub mod markdown;

/// Render format to export book highlights.
pub trait Render {
    /// Render book into specified output.
    fn render<W>(&mut self, book: &Book, out: &mut W) -> std::io::Result<()>
    where
        W: Write;

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
