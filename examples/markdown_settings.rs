use std::fs::File;

use highlights::highlights::examples::basic_attributes;
use highlights::render::markdown::{MarkdownRenderer, RenderSettings};
use highlights::render::Render;

/// Render markdown highlights with different settings.
fn main() {
    let mut render_settings = RenderSettings::default();
    render_settings.disable_split_lines();

    let mut renderer = MarkdownRenderer::new(render_settings);
    let output = File::create("target/example-settings.md").unwrap();
    let book = basic_attributes();
    renderer
        .render(&book, output)
        .expect("cannot render markdown highlights");
}
