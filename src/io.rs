use std::fs::File;
use std::io::{stdin, stdout, Read, Write};
use std::path::PathBuf;

use highlights::error::HighlightError;

pub fn output(out_arg: Option<PathBuf>) -> Result<Box<dyn Write>, HighlightError> {
    match out_arg {
        Some(path_buf) => {
            let path = path_buf.as_path();
            let output_file = File::create(path).map_err(|e| {
                HighlightError::new(format!("cannot write to file: {}", path.display()), e)
            })?;
            Ok(Box::new(output_file))
        }
        None => Ok(Box::new(stdout())),
    }
}

pub fn input(in_arg: Option<PathBuf>) -> Result<Box<dyn Read>, HighlightError> {
    match in_arg {
        Some(path_buf) => {
            let path = path_buf.as_path();
            let input_file = File::open(path).map_err(|e| {
                HighlightError::new(format!("cannot read input file: {}", path.display()), e)
            })?;
            Ok(Box::new(input_file))
        }
        None => Ok(Box::new(stdin())),
    }
}
