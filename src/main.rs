use std::path::PathBuf;

use clap::Parser;

use highlights::input::bookcision::JsonBook;
use highlights::input::HighlightsRead;
use highlights::render::markdown::MarkdownRenderer;
use highlights::render::Render;

mod io;

#[derive(Parser)]
#[command(name = "highlights")]
#[command(about = "Convert kindle highlights to markdown")]
#[command(version = "v0.2.0")]
struct Cli {
    #[arg(help = "input file")]
    source: Option<PathBuf>,
    #[arg(help = "output file")]
    target: Option<PathBuf>,
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    let mut input = io::input(cli.source);
    let book = JsonBook::from_reader(&mut input).unwrap().into();

    let mut out = io::output(cli.target);

    let mut renderer = MarkdownRenderer::default();
    renderer.render(&book, &mut out)
}
