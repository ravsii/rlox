mod scanner;

use scanner::Scanner;
use std::{
    env::args,
    fs,
    io::{self, Write},
    path::Path,
};

fn main() {
    let args: Vec<String> = args().collect();

    println!("All args: {:?}", args);

    match args.len() {
        1 => run_prompt(),
        2 => run_file(Path::new(&args[1])),
        _ => {
            println!("Usage: rlox [file]");
            std::process::exit(64);
        }
    }
}

fn run_file(path: &Path) {
    let contents = fs::read_to_string(path).unwrap_or_else(|err| {
        panic!(
            "failed to read file {:?}: {:?}",
            path.file_name().unwrap(),
            err
        )
    });

    run(contents);
}

fn run_prompt() {
    let mut input = String::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                println!("got EOF, closing...");
                return;
            }
            Ok(_) => {
                run(input.trim().to_string());
                input.clear();
            }
            Err(err) => {
                println!("failed to read line: {}", err);
                return;
            }
        }
    }
}

fn run(source: String) {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{}", token);
    }
}
