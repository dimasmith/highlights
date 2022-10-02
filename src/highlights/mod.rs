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

/// Highlighted part or passage in the book.
///
/// Each passage is always related to some location in the book.
/// The highlight can have the quote from the original text and readers comment.
#[derive(Clone, Debug)]
pub enum Highlight {
    /// Word-by-word quote from the original text.
    Quote { quote: String, location: Location },

    /// Margin note for the particular book location.
    Note { note: String, location: Location },

    /// Quote from the book and readers comment on the quote.
    Comment {
        quote: String,
        note: String,
        location: Location,
    },
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
    ///     [Highlight::Quote{
    ///         quote: "An ultimate answer is 42".to_owned(),
    ///         location: Location::new(42, "https://ultimate.answers.org/42")
    ///     }]);
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
    /// Convenience constructor for note only highlight.
    pub fn note<S>(note: S, location: Location) -> Self
    where
        S: Into<String>,
    {
        Highlight::Note {
            note: note.into(),
            location,
        }
    }

    /// Convenience constructor for quote.
    pub fn quote<S>(quote: S, location: Location) -> Self
    where
        S: Into<String>,
    {
        Highlight::Quote {
            quote: quote.into(),
            location,
        }
    }

    /// Convenience constructor for comment on quote.
    pub fn comment<S>(quote: S, note: S, location: Location) -> Self
    where
        S: Into<String>,
    {
        Highlight::Comment {
            quote: quote.into(),
            note: note.into(),
            location,
        }
    }

    pub fn location(&self) -> Location {
        let location = match self {
            Highlight::Note {
                note: _,
                location: l,
            } => l,
            Highlight::Quote {
                quote: _,
                location: l,
            } => l,
            Highlight::Comment {
                quote: _,
                note: _,
                location: l,
            } => l,
        };
        location.clone()
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
