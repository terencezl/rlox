use anyhow::Result;
use rlox::interpreter::Interpreter;
use rlox::parser::Parser;
use rlox::scanner::Scanner;
use std::io::{stdin, stdout, Write};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("ParserError")]
    ParserError,
    #[error("RuntimeError")]
    RuntimeError,
}

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() > 2 {
        println!("Usage: rlox [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1])?;
    } else {
        run_prompt()?;
    }
    Ok(())
}

fn run_file(path: &str) -> Result<()> {
    let interpreter = Interpreter::new();
    let source = std::fs::read_to_string(path)?;
    if let Err(e) = run(&source, &interpreter) {
        match e {
            Error::ParserError => std::process::exit(65),
            Error::RuntimeError => std::process::exit(70),
        }
    }
    Ok(())
}

fn run_prompt() -> Result<()> {
    println!("Welcome to 🐟rlox🐟 REPL!");
    let mut error = false;
    let prefix = "🐟> ";
    let bad_prefix = "😵> ";
    let interpreter = Interpreter::new();
    loop {
        print!("{}", if !error { prefix } else { bad_prefix });
        stdout().flush()?;
        let mut line = String::new();
        stdin().read_line(&mut line)?;
        if line.is_empty() {
            break;
        }
        match run(&line, &interpreter) {
            Ok(_) => error = false,
            Err(_) => error = true,
        }
    }
    Ok(())
}

fn run(source: &str, interpreter: &Interpreter) -> Result<(), Error> {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    // for token in tokens.iter() {
    //     println!("{}", token);
    // }

    let parser = Parser::new(&tokens);
    let expression = parser.parse().map_err(|e| {
        eprintln!("ParserError: {e}");
        Error::ParserError
    })?;
    // println!("{}", expression);

    let value = interpreter.interpret(&expression).map_err(|e| {
        eprintln!("RuntimeError: {e}");
        Error::RuntimeError
    })?;
    println!("{}", value);

    Ok(())
}
