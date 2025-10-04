use crate::{
    LoxRunner,
    ast::{Binary, Expr},
    token::{Token, TokenType},
};

pub struct Parser<'a> {
    lox_runner: &'a mut LoxRunner,
    tokens: Vec<Token>,
    current: usize,
}

impl<'a> Parser<'a> {
    fn new(lox_runner: &'a mut LoxRunner, tokens: Vec<Token>) -> Parser<'a> {
        Parser {
            lox_runner,
            tokens,
            current: 0,
        }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.match_type(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }

        expr
    }

    fn comparison(&self) -> Expr {
        todo!()
    }

    fn match_type(&mut self, types: &[TokenType]) -> bool {
        for typ in types {
            if self.check(*typ) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, typ: TokenType) -> bool {
        if self.is_end() {
            return false;
        };

        self.peek().typ() == typ
    }

    fn advance(&mut self) -> Token {
        if !self.is_end() {
            self.current += 1
        };

        self.previous()
    }

    fn is_end(&self) -> bool {
        self.peek().typ() == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1).unwrap().clone()
    }
}
