mod detect;

use std::env;
use std::fs;
use std::path::Path;
use detect::{detect_type, FileType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: lil_parser <path_to_binary>");
        std::process::exit(1);
    }

    let file_path = &args[1];
    let file_name = Path::new(&args[1])
    .file_name()
    .and_then(|s| s.to_str())
    .unwrap_or(&args[1]);

    let buffer: Vec<u8> = fs::read(file_path)?;

    let arch = match detect_type(&buffer) {
        FileType::MachO(arch) => format!("Mach-O: {}", arch),
        FileType::ELF => "ELF".to_string(),
        FileType::PE => "PE".to_string(),
        FileType::Another => "Another format".to_string(),
    };
    println!("file {} is {}", file_name, arch);
    Ok(())
}
