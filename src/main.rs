mod ast;
mod ast_printer;
mod interpreter;
mod parser;
mod scanner;
mod token;

use scanner::Scanner;
use std::{
    env::args,
    fs,
    io::{self, Write},
    path::Path,
};

use crate::{
    interpreter::Interpreter,
    parser::Parser,
    token::{Token, TokenType},
};

fn main() {
    let args: Vec<String> = args().collect();
    let mut runner = LoxRunner;

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
pub struct LoxRunner;

impl LoxRunner {
    fn run_file(&mut self, path: &Path) {
        let contents = fs::read_to_string(path).unwrap_or_else(|err| {
            panic!(
                "failed to read file {:?}: {:?}",
                path.file_name().unwrap(),
                err
            )
        });

        self.run(contents);
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
                }
                Err(err) => {
                    println!("failed to read line: {}", err);
                    return;
                }
            }
        }
    }

    fn run(&mut self, source: String) {
        let scanner = Scanner::new(source);
        let interpreter = Interpreter;

        let tokens = match scanner.scan_tokens() {
            Ok(tokens) => tokens,
            Err(err) => {
                self.error(0, &err.0);
                std::process::exit(65);
            }
        };

        let statements = match Parser::new(tokens).parse() {
            Ok(expr) => expr,
            Err(err) => {
                self.error_token(err.token, &err.message);
                std::process::exit(65);
            }
        };

        match interpreter.interpret(statements) {
            Ok(_) => {}
            Err(err) => {
                self.error_token(err.operator, &err.message);
                std::process::exit(70);
            }
        }
    }

    pub fn error(&mut self, line: i32, message: &str) {
        self.report(line, "", message);
    }

    pub fn error_token(&mut self, token: Token, message: &str) {
        match token.token_type {
            TokenType::Eof => self.report(token.line, " at end", message),
            _ => {
                let mut pos_str = String::from("at '");
                pos_str.push_str(token.lexeme.as_str());
                pos_str.push('\'');
                self.report(token.line, &pos_str, message);
            }
        }
    }

    fn report(&mut self, line: i32, pos: &str, message: &str) {
        println!("[line {}] Error {}: {}", line, pos, message);
    }
}
