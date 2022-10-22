//! Converts highlights from bookcision json format to markdown
use highlights::highlights::Book;
use highlights::input::bookcision::JsonBook;
use highlights::input::HighlightsRead;
use highlights::render::markdown::MarkdownRenderer;
use highlights::render::Render;
use std::fs::File;

pub fn main() {
    let bookcision_file = File::open("examples/bookcision.json").expect("input file not found");
    let bookcision_book = JsonBook::from_reader(bookcision_file).expect("cannot convert book");
    let book = Book::from(bookcision_book);

    let markdown_file =
        File::create("target/default_markdown.md").expect("cannot create output file");
    let mut renderer = MarkdownRenderer::default();
    renderer
        .render(&book, markdown_file)
        .expect("cannot render markdown document")
}
