use larp_script::interpreter::Interpreter;
use std::{env, fs, process};

fn main() {
    let mut args = env::args().skip(1);
    let Some(path) = args.next() else {
        eprintln!("Usage: larp <file.larp>");
        process::exit(1);
    };
    if args.next().is_some() {
        eprintln!("Error: provide exactly one source file.");
        process::exit(1);
    }
    if !path.ends_with(".larp") {
        eprintln!("Error: LarpScript files must use the .larp extension.");
        process::exit(1);
    }
    let source = fs::read_to_string(&path).unwrap_or_else(|error| {
        eprintln!("Could not read '{path}': {error}");
        process::exit(1);
    });
    let mut interpreter = Interpreter::new();
    if let Err(error) = interpreter.run_source(&source) {
        eprintln!("{error}");
        process::exit(1);
    }
    for line in interpreter.take_output() {
        println!("{line}");
    }
}

// cargo run -- .\examples\hello.larp
