//! Sample data entries to quickly test the system.
use crate::highlights::{Book, Highlight, Location};

/// Example of a book on chess and life by Garry Kasparov
pub fn chess_book() -> Book {
    Book::new(
        "How Life Imitates Chess: Making the Right Moves, from the Board to the Boardroom",
        "Garry Kasparov",
        [
            Highlight::quote(
                "the reality is that we discard our decisions almost as soon as we make them",
                Location::new(157, "kindle://book?action=open&asin=B0049U443Q&location=157"),
            ),
            Highlight::note(
                "Create a personalized map of your decision-making process",
                Location::new(294, "kindle://book?action=open&asin=B0049U443Q&location=294"),
            ),
            Highlight::comment(
                "Drawing it as an actual map might be fun",
                "The map tells you which areas of your mind are well-known to you and which are still uncharted.",
                Location::new(295, "kindle://book?action=open&asin=B0049U443Q&location=295"),
            ),
        ],
    )
}

/// Example with basic attributes
pub fn basic_attributes() -> Book {
    Book::new(
        "Title",
        "Author",
        [
            Highlight::quote("Quote_1", Location::new(1, "book://test?location=1")),
            Highlight::note("Note_2", Location::new(2, "book://test?location=2")),
            Highlight::comment(
                "Quote_3",
                "Note_3",
                Location::new(3, "book://test?location=3"),
            ),
        ],
    )
}

/// Example from imaginary rustonomicon
pub fn rustonomicon() -> Book {
    Book::new(
        "Rustonomicon",
        "The Rust Community",
        [
            Highlight::quote("This is a helpful quote from the Rustonomicon", Location::new(157, "kindle://book?action=open&asin=c3dd2fe1-0cc9-42d1-b11f-da858a10f1de&location=157")),
            Highlight::note("Note to self: don't forget to lint your project.", Location::new(693, "kindle://book?action=open&asin=c3dd2fe1-0cc9-42d1-b11f-da858a10f1de&location=693")),
            Highlight::comment(
                "This is an interesting quote from the Rustonomicon",
                "This quote needs clarification, so that's why the note is here",
                Location::new(305, "kindle://book?action=open&asin=c3dd2fe1-0cc9-42d1-b11f-da858a10f1de&location=305"),
            ),
        ],
    )
}
