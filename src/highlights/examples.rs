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
