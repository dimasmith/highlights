use highlights::highlights::examples;
use highlights::render::markdown::MarkdownRenderer;
use highlights::render::Render;

mod io;

fn main() -> std::io::Result<()> {
    let mut out = io::output(std::env::args().nth(1));
    let book = examples::chess_book();
    let mut renderer = MarkdownRenderer::default();
    renderer.render(&book, &mut out)
}
