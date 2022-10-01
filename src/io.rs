use std::fs::File;
use std::io::{stdout, Write};

pub fn output(out_arg: Option<String>) -> Box<dyn Write> {
    match out_arg {
        Some(path) => Box::new(File::create(path).expect("cannot create file")),
        None => Box::new(stdout()),
    }
}
