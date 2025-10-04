use crate::ast::{Expr, Literal};

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(expr: &Expr) -> String {
        match expr {
            Expr::Binary(binary) => format!(
                "({} {} {})",
                binary.operator.lexeme,
                AstPrinter::print(&binary.left),
                AstPrinter::print(&binary.right)
            ),
            Expr::Grouping(grouping) => {
                format!("(group {})", AstPrinter::print(&grouping.expression))
            }
            Expr::Literal(literal) => match literal {
                Literal::Number(n) => n.to_string(),
                Literal::String(s) => s.clone(),
                Literal::Bool(b) => b.to_string(),
                Literal::Nil => "nil".to_string(),
            },
            Expr::Unary(unary) => {
                format!(
                    "({} {})",
                    unary.operator.lexeme,
                    AstPrinter::print(&unary.right)
                )
            }
        }
    }
}

pub struct AstPrinterRPN;

impl AstPrinterRPN {
    pub fn print(expr: &Expr) -> String {
        match expr {
            Expr::Binary(binary) => {
                AstPrinterRPN::parenthesize(&binary.operator.lexeme, &[&binary.left, &binary.right])
            }
            Expr::Grouping(grouping) => {
                AstPrinterRPN::parenthesize("group", &[&grouping.expression])
            }
            Expr::Literal(literal) => match literal {
                Literal::Number(n) => n.to_string(),
                Literal::String(s) => s.clone(),
                Literal::Bool(b) => b.to_string(),
                Literal::Nil => "nil".to_string(),
            },
            Expr::Unary(unary) => {
                AstPrinterRPN::parenthesize(&unary.operator.lexeme, &[&unary.right])
            }
        }
    }

    fn parenthesize(name: &str, exprs: &[&Expr]) -> String {
        let mut result = String::new();

        result.push('(');
        for expr in exprs {
            result.push_str(&AstPrinterRPN::print(expr));
            result.push(' ');
        }

        result.push_str(name);
        result.push(')');
        result
    }
}
