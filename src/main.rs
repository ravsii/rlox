mod ast;
mod ast_printer;
mod parser;
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

use crate::{
    ast::{Binary, Expr, Grouping, Literal, Unary},
    ast_printer::AstPrinter,
    parser::Parser,
    token::{Token, TokenType},
};

fn main() {
    let args: Vec<String> = args().collect();
    let mut runner = LoxRunner::default();

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
pub struct LoxRunner {
    had_error: bool,
}

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

    fn run(&mut self, source: String) {
        let mut scanner = Scanner::new(self, source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(self, tokens);
        let expr = parser.parse();

        // TODO: remove all these errors stuff and just return errors from functions
        if self.had_error || expr.is_none() {
            return;
        }

        println!("{}", AstPrinter::print(&expr.unwrap()));
    }

    pub fn error(&mut self, line: i32, message: &str) {
        self.report(line, "", message);
    }

    pub fn error_token(&mut self, token: Token, message: &str) {
        match token.token_type {
            TokenType::EOF => self.report(token.line, " at end", message),
            _ => {
                let mut pos_str = String::from(" at '");
                pos_str.push_str(token.lexeme.as_str());
                self.report(token.line, &pos_str, message);
            }
        }
    }

    fn report(&mut self, line: i32, pos: &str, message: &str) {
        println!("[line {}] Error {}: {}", line, pos, message);
        self.had_error = true
    }
}
