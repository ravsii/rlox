use crate::{
    ast::{Binary, Expr, Literal, Unary},
    token::TokenType,
};

pub struct Interpreter;

impl Interpreter {
    pub fn evaluate(&self, expr: Expr) -> Literal {
        match expr {
            Expr::Binary(binary) => self.eval_binary(binary),
            Expr::Grouping(grouping) => self.evaluate(*grouping.expression),
            Expr::Literal(literal) => literal,
            Expr::Unary(unary) => self.eval_unary(unary),
        }
    }

    fn eval_binary(&self, binary: Binary) -> Literal {
        let left = self.evaluate(*binary.left);
        let right = self.evaluate(*binary.right);

        match binary.operator.token_type {
            TokenType::Minus => match (left, right) {
                (Literal::Number(l), Literal::Number(r)) => Literal::Number(l - r),
                _ => Literal::Nil,
            },
            TokenType::Slash => match (left, right) {
                (Literal::Number(_), Literal::Number(0.0)) => Literal::Nil,
                (Literal::Number(l), Literal::Number(r)) => Literal::Number(l / r),
                _ => Literal::Nil,
            },
            TokenType::Star => match (left, right) {
                (Literal::Number(l), Literal::Number(r)) => Literal::Number(l * r),
                _ => Literal::Nil,
            },
            TokenType::Plus => match (left, right) {
                (Literal::Number(l), Literal::Number(r)) => Literal::Number(l + r),
                (Literal::String(l), Literal::String(r)) => Literal::String(format!("{}{}", l, r)),
                _ => Literal::Nil,
            },
            TokenType::Greater => match (left, right) {
                (Literal::Number(l), Literal::Number(r)) => Literal::Bool(l > r),
                _ => Literal::Nil,
            },
            TokenType::GreaterEqual => match (left, right) {
                (Literal::Number(l), Literal::Number(r)) => Literal::Bool(l >= r),
                _ => Literal::Nil,
            },
            TokenType::Less => match (left, right) {
                (Literal::Number(l), Literal::Number(r)) => Literal::Bool(l < r),
                _ => Literal::Nil,
            },
            TokenType::LessEqual => match (left, right) {
                (Literal::Number(l), Literal::Number(r)) => Literal::Bool(l <= r),
                _ => Literal::Nil,
            },
            TokenType::BangEqual => Literal::Bool(!self.is_equal(left, right)),
            TokenType::EqualEqual => Literal::Bool(self.is_equal(left, right)),

            _ => Literal::Nil,
        }
    }

    fn eval_unary(&self, unary: Unary) -> Literal {
        let right = self.evaluate(*unary.right);

        if unary.operator.token_type == TokenType::Minus {
            return match right {
                Literal::Number(n) => Literal::Number(-n),
                _ => Literal::Nil,
            };
        }
        if unary.operator.token_type == TokenType::Bang {
            return Literal::Bool(!self.is_truthy(right));
        }

        Literal::Nil
    }

    fn is_equal(&self, left: Literal, right: Literal) -> bool {
        left == right
    }

    fn is_truthy(&self, literal: Literal) -> bool {
        match literal {
            Literal::Bool(b) => b,
            Literal::Nil => false,
            _ => true,
        }
    }
}
