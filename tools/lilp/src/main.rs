mod byteview;
mod reader;
mod error;
mod metadata;
mod fileformat;
mod binary;
mod elf;

use std::{env, fs, process};
use byteview::ByteView;
use binary::Binary;

fn main() {
    let path = match env::args().nth(1) {
        Some(p) => p,
        None => {
            eprintln!("Usage: binparse <file>");
            process::exit(1);
        }
    };

    let data = match fs::read(&path) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Error reading '{}': {}", path, e);
            process::exit(1);
        }
    };

    let view = ByteView::new(&data);

    match Binary::parse(view) {
        Ok(binary) => print_info(&path, binary.as_format()),
        Err(e) => {
            eprintln!("Error parsing '{}': {:?}", path, e);
            process::exit(1);
        }
    }
}

fn print_info(path: &str, fmt: &dyn crate::fileformat::FileFormat) {
    println!("{}: {}", path, fmt.name());
    println!();
    for (key, value) in fmt.metadata().fields() {
        println!("  {:<12} {}", key, value);
    }
}
