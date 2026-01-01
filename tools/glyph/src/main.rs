use std::fs;
use std::env;
use glyph::{parse_glyph, verify};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: glyph-cli <command> <file> [code_file]");
        return;
    }
    let command = &args[1];
    let file = &args[2];
    match command.as_str() {
        "parse" => {
            let content = match fs::read_to_string(file) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error reading file {}: {}", file, e);
                    return;
                }
            };
            match parse_glyph(&content) {
                Ok((_, spec)) => println!("{:?}", spec),
                Err(e) => eprintln!("Parse error: {:?}", e),
            }
        }
        "verify" => {
            if args.len() < 4 {
                eprintln!("Usage: glyph-cli verify <spec_file> <code_file>");
                return;
            }
            let spec_file = file;
            let code_file = &args[3];
            let spec_content = match fs::read_to_string(spec_file) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error reading spec {}: {}", spec_file, e);
                    return;
                }
            };
            let spec = match parse_glyph(&spec_content) {
                Ok((_, s)) => s,
                Err(e) => {
                    eprintln!("Parse error: {:?}", e);
                    return;
                }
            };
            match verify(&spec, code_file) {
                Ok(result) => println!("{}", result),
                Err(e) => println!("FAIL: {}", e),
            }
        }
        _ => eprintln!("Unknown command"),
    }
}