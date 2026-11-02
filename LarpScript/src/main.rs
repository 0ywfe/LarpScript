use larp_script::evaluator::evaluate_expression;
use larp_script::parser::parse_expression;
use std::{env, fs, process};

fn main() {
    let Some(path) = env::args().nth(1) else {
        eprintln!("Usage: cargo run -- <file.larp>");
        process::exit(1);
    };
    let source = fs::read_to_string(&path).unwrap_or_else(|error| {
        eprintln!("Could not read file: {error}");
        process::exit(1);
    });
    let mut pairs = parse_expression(&source).unwrap_or_else(|error| {
        eprintln!("Syntax error:\n{error}");
        process::exit(1);
    });
    let expr = pairs
        .next()
        .and_then(|p| p.into_inner().next())
        .expect("Expected an expression");
    let value = evaluate_expression(expr).unwrap_or_else(|err| {
        eprintln!("Runtime error: {err:?}");
        std::process::exit(1);
    });
    println!("{value:?}");
}
