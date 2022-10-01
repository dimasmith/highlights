//! Import highlights from bookcision json format.

use std::error::Error;
use std::io::Read;

use serde::{Deserialize, Serialize};

use crate::highlights::{Book, Highlight, Location};
use crate::input::HighlightsRead;

/// JSON representation of bookcision kindle highlights export.
#[derive(Serialize, Deserialize, Debug)]
pub struct JsonBook {
    asin: String,
    title: String,
    authors: String,
    highlights: Vec<JsonHighlight>,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonHighlight {
    text: String,
    location: JsonHighlightLocation,
    note: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonHighlightLocation {
    value: u32,
    url: String,
}

impl From<JsonBook> for Book {
    /// Convert json representation of highlights into a book.
    fn from(json: JsonBook) -> Self {
        let highlights = json.highlights.iter().map(|h| {
            let location = &h.location;
            Highlight::new(
                h.text.clone(),
                Location::new(location.value as usize, location.url.clone()),
            )
        });
        Book::new(json.title.clone(), json.authors.clone(), highlights)
    }
}

impl HighlightsRead for JsonBook {
    fn from_reader<R>(reader: &mut R) -> Result<Self, Box<dyn Error>>
    where
        R: Read,
    {
        let b = serde_json::from_reader(reader)?;
        Ok(b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_basic_details() {
        let book_json = r#"
        {
            "asin": "B0049U443Q",
            "title": "How Life Imitates Chess: Making the Right Moves, from the Board to the Boardroom",
            "authors": "Garry Kasparov",
            "highlights": []
        }
        "#;
        let book = JsonBook::from_str(book_json);
        assert_eq!("Garry Kasparov", book.authors);
    }

    #[test]
    fn parse_highlights() {
        let book_json = r#"{
  "asin": "B0049U443Q",
  "title": "How Life Imitates Chess: Making the Right Moves, from the Board to the Boardroom",
  "authors": "Garry Kasparov",
  "highlights": [
    {
      "text": "the reality is that we discard our decisions almost as soon as we make them. Too often we just live with the results and move on, repeating the same flawed process with the same flawed results.",
      "isNoteOnly": false,
      "location": {
        "url": "kindle://book?action=open&asin=B0049U443Q&location=157",
        "value": 157
      },
      "note": null
    },
    {
      "text": "“Why this move? What am I trying to achieve and how does this move help me achieve it?”",
      "isNoteOnly": false,
      "location": {
        "url": "kindle://book?action=open&asin=B0049U443Q&location=447",
        "value": 447
      },
      "note": "Each move or decision should contribute to some strategical objective"
    }
  ]
}"#;
        let book = JsonBook::from_str(book_json);
        assert_eq!("Garry Kasparov", book.authors);
        assert_eq!(2, book.highlights.len());
    }

    impl JsonBook {
        fn from_str(value: &str) -> Self {
            serde_json::from_str(value).unwrap()
        }
    }
}