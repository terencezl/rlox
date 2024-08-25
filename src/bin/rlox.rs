use anyhow::Result;
use rlox::scanner::Scanner;
use std::io::{stdin, stdout, Write};

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
    let source = std::fs::read_to_string(path)?;
    if let Err(_) = run(&source) {
        std::process::exit(65);
    }
    Ok(())
}

fn run_prompt() -> Result<()> {
    println!("Welcome to 🐟rlox🐟 REPL!");
    loop {
        print!("🐟> ");
        stdout().flush()?;
        let mut line = String::new();
        stdin().read_line(&mut line)?;
        if line.is_empty() {
            break;
        }
        let _ = run(&line);
    }

    Ok(())
}

fn run(source: &str) -> Result<()> {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    for token in tokens? {
        println!("{}", token);
    }
    Ok(())
}
