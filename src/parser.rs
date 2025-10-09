use crate::{
    ast::{Expr, Literal, Stmt},
    token::{Token, TokenType},
};

#[derive(Debug, Clone)]
pub struct ParseError {
    pub token: Token,
    pub message: String,
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = Vec::new();
        while !self.is_end() {
            statements.push(self.statement()?);
        }

        Ok(statements)
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_type(&[TokenType::Print]) {
            return self.print_statement();
        }

        self.expression_statement()
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;

        Ok(Stmt::Print(expr))
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;

        Ok(Stmt::Print(expr))
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;

        while self.match_type(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::new_binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;

        while self.match_type(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::new_binary(expr, operator, right)
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        while self.match_type(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::new_binary(expr, operator, right)
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        while self.match_type(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::new_binary(expr, operator, right)
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_type(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::new_unary(operator, right));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_type(&[TokenType::False]) {
            return Ok(Expr::Literal(Literal::Bool(false)));
        }
        if self.match_type(&[TokenType::True]) {
            return Ok(Expr::Literal(Literal::Bool(true)));
        }
        if self.match_type(&[TokenType::Nil]) {
            return Ok(Expr::Literal(Literal::Nil));
        }

        if self.match_type(&[TokenType::Number, TokenType::String]) {
            let prev = self.previous().literal;
            return Ok(Expr::Literal(prev.unwrap()));
        }

        if self.match_type(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression")?;
            return Ok(Expr::new_grouping(expr));
        }

        Err(ParseError {
            token: self.peek(),
            message: String::from("Expected expression"),
        })
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

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<(), ParseError> {
        if !self.check_token(token_type) {
            return Err(ParseError {
                token: self.peek(),
                message: message.to_string(),
            });
        };

        self.advance();

        Ok(())
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
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.current).unwrap().clone()
    }

    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1).unwrap().clone()
    }
}
