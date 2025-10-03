use crate::token::{Token, TokenType};

#[derive(Default)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: i32,
    current: i32,
    line: i32,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            line: 1,
            ..Default::default()
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_end() {
            self.start = self.current;
            self.scan_token();
        }

        let line = 1;

        self.tokens
            .push(Token::new(TokenType::Eof, "".into(), "".into(), line));
        return &self.tokens;
    }

    fn scan_token(&self) {}

    fn is_end(&self) -> bool {
        self.current >= self.source.len() as i32
    }
}
