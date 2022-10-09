use std::fs::File;

use highlights::highlights::examples::basic_attributes;
use highlights::render::markdown::{MarkdownRenderer, RenderSettings};
use highlights::render::Render;

/// Render markdown highlights with different settings.
fn main() {
    let render_settings = RenderSettings::new().disable_split_lines().build();

    let mut renderer = MarkdownRenderer::new(render_settings);
    let output = File::create("target/example-settings.md").unwrap();
    let book = basic_attributes();
    renderer
        .render(&book, output)
        .expect("cannot render markdown highlights");
}
