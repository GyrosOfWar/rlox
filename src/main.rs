use scanner::Scanner;
use std::{
    env,
    io::{stdin, BufRead},
    path::Path,
    process,
};
use thiserror::Error;

mod scanner;
mod token;

#[derive(Error, Debug)]
pub enum Error {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

fn run(source: &str) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan();
}

fn run_file(path: impl AsRef<Path>) -> Result<()> {
    let source = std::fs::read_to_string(path)?;
    run(&source);
    Ok(())
}

fn run_repl() -> Result<()> {
    let stdin = stdin();
    let mut stdin = stdin.lock();
    let mut line = String::new();
    loop {
        print!("> ");
        stdin.read_line(&mut line)?;
        run(&line);
        line.clear();
    }
}

fn main() -> Result<()> {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.len() > 1 {
        println!("usage: rlox [script]");
        process::exit(64);
    } else if args.len() == 1 {
        run_file(&args[1])
    } else {
        run_repl()
    }
}
