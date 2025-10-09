use std::fmt;

use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Stmt {
    Print(Expr),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
    Varibale(Varibale),
}

impl Expr {
    pub fn new_binary(left: Expr, operator: Token, right: Expr) -> Expr {
        Expr::Binary(Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    pub fn new_grouping(expr: Expr) -> Expr {
        Expr::Grouping(Grouping {
            expression: Box::new(expr),
        })
    }

    pub fn new_unary(operator: Token, right: Expr) -> Expr {
        Expr::Unary(Unary {
            operator,
            right: Box::new(right),
        })
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
pub struct Varibale {
    pub name: Token,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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
