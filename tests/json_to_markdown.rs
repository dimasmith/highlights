use highlights::input::bookcision::JsonBook;
use highlights::input::HighlightsRead;
use highlights::render::markdown::MarkdownRenderer;
use highlights::render::Render;
use std::fs::File;

#[test]
fn convert_bookcision_json_to_markdown() {
    let input_file = File::open("tests/rustonomicon.json").unwrap();
    let book = JsonBook::from_reader(input_file).unwrap().into();
    let mut output: Vec<u8> = vec![];
    let mut renderer = MarkdownRenderer::default();
    renderer.render(&book, &mut output).unwrap();

    let markdown = String::from_utf8(output).unwrap();

    assert!(markdown.contains("# Rustonomicon"), "missing book title");
}
