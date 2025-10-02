mod scanner;
mod token;

use scanner::Scanner;
use std::{
    env::args,
    fs,
    io::{self, Write},
    path::Path,
    process,
};

fn main() {
    let args: Vec<String> = args().collect();

    println!("All args: {:?}", args);
    let runner = LoxRunner::default();

    match args.len() {
        1 => runner.run_prompt(),
        2 => runner.run_file(Path::new(&args[1])),
        _ => {
            println!("Usage: rlox [file]");
            std::process::exit(64);
        }
    }
}

#[derive(Default)]
struct LoxRunner {
    had_error: bool,
}

impl LoxRunner {
    fn run_file(&self, path: &Path) {
        let contents = fs::read_to_string(path).unwrap_or_else(|err| {
            panic!(
                "failed to read file {:?}: {:?}",
                path.file_name().unwrap(),
                err
            )
        });

        self.run(contents);
        if self.had_error {
            process::exit(65);
        }
    }

    fn run_prompt(mut self) {
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
                    self.run(input.trim().to_string());
                    input.clear();
                    self.had_error = false
                }
                Err(err) => {
                    println!("failed to read line: {}", err);
                    return;
                }
            }
        }
    }

    fn run(&self, source: String) {
        let scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        for token in tokens {
            println!("{}", token);
        }
    }

    fn error(self, line: i32, message: &str) {
        self.report(line, "", message);
    }

    fn report(mut self, line: i32, pos: &str, message: &str) {
        println!("[line {}] Error {}: {}", line, pos, message);
        self.had_error = true
    }
}
