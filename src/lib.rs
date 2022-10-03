//! Convert kindle highlights to a different formats.
//!
//! ## Motivation
//!
//! Placing highlights from Kindle to any note taking software requires converting to other formats.
//! This package helps converting json format exported with the help of [bookcision](https://readwise.io/bookcision)
//! to Markdown format suitable to placing into Obsidian, Notion and the likes.
//!
//! ## Example
//!
//! Converting json file to markdown
//! ```no_run
//! # use std::fs::File;
//! # use highlights::input::bookcision::JsonBook;
//! # use highlights::input::HighlightsRead;
//! # use highlights::render::markdown::MarkdownRenderer;
//! # use highlights::render::Render;
//! #
//! let input_file = File::open("bookcision.json").unwrap();
//! let output_file = File::create("highlights.md").unwrap();
//!
//! let book = JsonBook::from_reader(input_file).unwrap().into();
//! let mut renderer = MarkdownRenderer::default();
//! renderer.render(&book, output_file).unwrap();
//!
//! ```
pub mod error;
pub mod highlights;
pub mod input;
pub mod render;
