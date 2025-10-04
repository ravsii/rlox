use std::fmt;

use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
}

pub trait ExprVisitor<R> {
    fn visit_binary(&mut self, expr: &Binary) -> R;
    fn visit_grouping(&mut self, expr: &Grouping) -> R;
    fn visit_literal(&mut self, expr: &Literal) -> R;
    fn visit_unary(&mut self, expr: &Unary) -> R;
}

impl Expr {
    pub fn accept<R>(&self, visitor: &mut dyn ExprVisitor<R>) -> R {
        match self {
            Expr::Binary(expr) => visitor.visit_binary(expr),
            Expr::Grouping(expr) => visitor.visit_grouping(expr),
            Expr::Literal(expr) => visitor.visit_literal(expr),
            Expr::Unary(expr) => visitor.visit_unary(expr),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Grouping {
    pub expression: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Number(n) => write!(f, "{}", n),
            Literal::String(s) => write!(f, "{}", s),
            Literal::Bool(b) => write!(f, "{}", b),
            Literal::Nil => write!(f, "nil"),
        }
    }
}
