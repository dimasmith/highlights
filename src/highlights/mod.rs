//! Data model for book highlights.
pub mod examples;

/// Book with highlighted passages.
///
/// Holds basic data on the book and a list of highlights.
#[derive(Clone, Debug)]
pub struct Book {
    title: String,
    authors: String,
    highlights: Vec<Highlight>,
}

/// Highlighted passage in the book.
///
/// Holds the data on the highlighted passage of the book.
/// Contains passage text and a link to the original text.
#[derive(Clone, Debug)]
pub struct Highlight {
    text: String,
    location: Location,
}

/// Location of highlighted passage.
///
/// Contains the numeric value of the passage as well as a link to reach the highlight.
#[derive(Clone, Debug)]
pub struct Location {
    value: usize,
    link: String,
}

impl Book {
    /// Create new book with highlights.
    ///
    /// ## Example:
    /// ```
    /// # use highlights::highlights::{Book, Highlight, Location};
    /// let hhgttg = Book::new(
    ///     "The Hitchhikers Guide",
    ///     "Douglas Adams",
    ///     [Highlight::new(
    ///         "An ultimate answer is 42",
    ///         Location::new(42, "https://ultimate.answers.org/42")
    ///     )]);
    /// ```
    pub fn new<S, I>(title: S, authors: S, highlights: I) -> Self
    where
        S: Into<String>,
        I: IntoIterator<Item = Highlight>,
    {
        Book {
            title: title.into(),
            authors: authors.into(),
            highlights: Vec::from_iter(highlights),
        }
    }

    /// Book title.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Book authors.
    pub fn authors(&self) -> &str {
        &self.authors
    }

    /// Book highlights.
    ///
    /// Returns a clone of the highlights vector.
    pub fn highlights(&self) -> Vec<Highlight> {
        self.highlights.clone()
    }
}

impl Highlight {
    /// Creates a new highlight with the highlighted text and location.
    pub fn new<S>(text: S, location: Location) -> Self
    where
        S: Into<String>,
    {
        Highlight {
            text: text.into(),
            location,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn location(&self) -> &Location {
        &self.location
    }
}

impl Location {
    /// Creates a new location with the numeric location value and the link.
    pub fn new<S>(value: usize, link: S) -> Self
    where
        S: Into<String>,
    {
        Location {
            value,
            link: link.into(),
        }
    }

    pub fn value(&self) -> usize {
        self.value
    }

    pub fn link(&self) -> &str {
        &self.link
    }
}
