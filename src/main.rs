use highlights::input::bookcision::JsonBook;
use highlights::input::HighlightsRead;
use highlights::render::markdown::MarkdownRenderer;
use highlights::render::Render;

mod io;

fn main() -> std::io::Result<()> {
    let mut input = io::input(std::env::args().nth(1));
    let book = JsonBook::from_reader(&mut input).unwrap().into();

    let mut out = io::output(std::env::args().nth(2));

    let mut renderer = MarkdownRenderer::default();
    renderer.render(&book, &mut out)
}
