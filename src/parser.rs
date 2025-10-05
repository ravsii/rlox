use crate::{
    LoxRunner,
    ast::{Binary, Expr, Literal},
    token::{Token, TokenType},
};

pub struct ParseError;

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
            expr = Expr::new_binary(expr, operator, right)
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.match_type(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term();
            expr = Expr::new_binary(expr, operator, right)
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.match_type(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Expr::new_binary(expr, operator, right)
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_type(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Expr::new_binary(expr, operator, right)
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_type(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            return Expr::new_unary(operator, right);
        }

        self.primary().unwrap()
    }

    fn primary(&mut self) -> Option<Expr> {
        if self.match_type(&[TokenType::False]) {
            return Some(Expr::Literal(Literal::Bool(false)));
        }
        if self.match_type(&[TokenType::True]) {
            return Some(Expr::Literal(Literal::Bool(true)));
        }
        if self.match_type(&[TokenType::Nil]) {
            return Some(Expr::Literal(Literal::Nil));
        }

        if self.match_type(&[TokenType::Number, TokenType::String]) {
            let prev = self.previous().literal;
            return Some(Expr::Literal(prev.unwrap()));
        }

        if self.match_type(&[TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression");
            return Some(Expr::new_grouping(expr));
        }

        self.error(self.peek(), "Expected expression");
        None
    }

    fn match_type(&mut self, types: &[TokenType]) -> bool {
        for typ in types {
            if self.check_token(*typ) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn consume(&mut self, token_type: TokenType, message: &str) {
        if !self.check_token(token_type) {
            self.error(self.peek().clone(), message)
        };

        self.advance();
    }

    fn error(&mut self, token: Token, message: &str) {
        self.lox_runner.error_token(token, message);
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::For
                | TokenType::Fun
                | TokenType::If
                | TokenType::Print
                | TokenType::Return
                | TokenType::Var
                | TokenType::While => {
                    return;
                }
                _ => {}
            }

            self.advance();
        }
    }

    fn check_token(&self, typ: TokenType) -> bool {
        if self.is_end() {
            return false;
        };

        self.peek().token_type == typ
    }

    fn advance(&mut self) -> Token {
        if !self.is_end() {
            self.current += 1
        };

        self.previous()
    }

    fn is_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.current).unwrap().clone()
    }

    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1).unwrap().clone()
    }
}
