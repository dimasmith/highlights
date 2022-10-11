use std::fs::File;

use highlights::highlights::examples::rustonomicon;
use highlights::render::markdown::{MarkdownRenderer, NoteStyle, QuoteStyle, RenderSettings};
use highlights::render::Render;

/// Render markdown highlights with different settings.
fn main() {
    let render_settings = RenderSettings::new()
        .enable_split_lines()
        .quote_style(QuoteStyle::BlockQuote)
        .note_style(NoteStyle::NestedQuote)
        .build();

    let mut renderer = MarkdownRenderer::new(render_settings);
    let output = File::create("target/example-settings.md").unwrap();
    let book = rustonomicon();
    renderer
        .render(&book, output)
        .expect("cannot render markdown highlights");
}
