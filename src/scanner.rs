use crate::{
    LoxRunner,
    token::{Literal, Token, TokenType},
};

pub struct Scanner<'a> {
    lox_runner: &'a mut LoxRunner,
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: i32,
}

impl<'a> Scanner<'a> {
    pub fn new(lox_runner: &'a mut LoxRunner, source: String) -> Scanner<'a> {
        Scanner {
            current: 0,
            line: 1,
            lox_runner,
            source,
            start: 0,
            tokens: vec![],
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "".into(), None, self.line));

        &self.tokens
    }

    fn scan_token(&mut self) {
        match self.advance() {
            // Simple tokens
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),

            // Special comment case
            '/' if self.char_match('/') => {
                while self.peek() != '\n' && !self.is_end() {
                    self.advance();
                }
            }
            '/' => self.add_token(TokenType::Slash),

            // One-Two character tokens
            '!' => {
                if self.char_match('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.char_match('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '<' => {
                if self.char_match('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            '>' => {
                if self.char_match('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }

            // Literals
            '"' => self.string(),
            '0'..='9' => self.number(),

            // Useless characters
            ' ' | '\r' | '\t' => {}

            '\n' => self.line += 1,

            c => self
                .lox_runner
                .error(self.line, format!("Unexpected character: {}", c).as_str()),
        }
    }

    fn is_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn cur_char(&self) -> char {
        self.source.chars().nth(self.current).unwrap()
    }

    fn advance(&mut self) -> char {
        let c = self.cur_char();
        self.current += 1;
        c
    }

    fn peek(&self) -> char {
        if self.is_end() {
            return '\0';
        }

        self.cur_char()
    }

    fn peek_next(&self) -> char {
        if self.current >= self.source.len() {
            return '\0';
        }

        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn char_match(&mut self, expected: char) -> bool {
        println!("1 {}", expected);
        if self.is_end() || self.cur_char() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(token_type, text.to_string(), None, self.line));
    }

    fn add_token_val(&mut self, token_type: TokenType, val: Literal) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(
            token_type,
            text.to_string(),
            Some(val),
            self.line,
        ));
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_end() {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.advance();
        }

        if self.is_end() {
            self.lox_runner.error(self.line, "Unterminated string");
            return;
        }

        self.advance();
        let text = &self.source[self.start + 1..self.current - 1];
        self.add_token_val(TokenType::String, Literal::String(text.into()));
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        self.add_token_val(
            TokenType::Number,
            Literal::Number(self.source[self.start..self.current].parse().unwrap()),
        );
    }
}
