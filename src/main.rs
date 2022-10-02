use std::path::PathBuf;

use clap::Parser;

use highlights::error::HighlightError;
use highlights::input::bookcision::JsonBook;
use highlights::input::HighlightsRead;
use highlights::render::markdown::MarkdownRenderer;
use highlights::render::Render;

mod io;

#[derive(Parser)]
#[command(name = "highlights")]
#[command(about = "Convert kindle highlights to markdown")]
#[command(version = "v0.3.0-dev")]
struct Cli {
    #[arg(help = "input file")]
    source: Option<PathBuf>,
    #[arg(help = "output file")]
    target: Option<PathBuf>,
}

fn main() {
    let result = convert_highlights();
    match result {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(74);
        }
    }
}

fn convert_highlights() -> Result<(), HighlightError> {
    let cli = Cli::parse();

    let input = io::input(cli.source)?;
    let book = JsonBook::from_reader(input)?.into();

    let out = io::output(cli.target)?;
    let mut renderer = MarkdownRenderer::default();
    renderer.render(&book, out)
}
