use std::fs::File;
use std::io::{stdin, stdout, Read, Write};
use std::path::PathBuf;

use highlights::HighlightError;

pub fn output(out_arg: Option<PathBuf>) -> Result<Box<dyn Write>, HighlightError> {
    match out_arg {
        Some(path) => {
            let output_file = File::create(path).map_err(HighlightError::from)?;
            Ok(Box::new(output_file))
        }
        None => Ok(Box::new(stdout())),
    }
}

pub fn input(in_arg: Option<PathBuf>) -> Result<Box<dyn Read>, HighlightError> {
    match in_arg {
        Some(path) => {
            let input_file = File::open(path).map_err(HighlightError::from)?;
            Ok(Box::new(input_file))
        }
        None => Ok(Box::new(stdin())),
    }
}
