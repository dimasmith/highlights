//! Various input formats to read highlights from.
use std::error::Error;
use std::io::Read;

use crate::highlights::Book;

pub mod bookcision;

/// Read highlights from input sources.
///
/// Each read object is guaranteed to be convertable to book highlights.
pub trait HighlightsRead: Into<Book> {
    /// Creates highlights from the input source.
    fn from_reader<R>(reader: &mut R) -> Result<Self, Box<dyn Error>>
    where
        R: Read;
}
