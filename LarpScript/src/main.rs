use larp_script::interpreter::Interpreter;
use std::{env, fs, process};

fn main() {
    let Some(path) = env::args().nth(1) else {
        eprintln!("Usage: cargo run -- <file.larp>");
        process::exit(1);
    };
    let source = fs::read_to_string(&path).unwrap_or_else(|error| {
        eprintln!("Could not read file '{path}': {error}");
        process::exit(1);
    });
    let mut interpreter = Interpreter::new();
    if let Err(error) = interpreter.run_source(&source) {
        eprintln!("Error: {error:?}");
        process::exit(1);
    }
    for line in interpreter.take_output() {
        println!("{line}");
    }
}

// cargo run -- .\examples\hello.larp
