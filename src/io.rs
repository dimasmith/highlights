use std::fs::File;
use std::io::{stdin, stdout, Read, Write};

pub fn output(out_arg: Option<String>) -> Box<dyn Write> {
    match out_arg {
        Some(path) => Box::new(File::create(path).expect("cannot create file")),
        None => Box::new(stdout()),
    }
}

pub fn input(in_arg: Option<String>) -> Box<dyn Read> {
    match in_arg {
        Some(path) => Box::new(File::open(path).expect("cannot read file")),
        None => Box::new(stdin()),
    }
}
