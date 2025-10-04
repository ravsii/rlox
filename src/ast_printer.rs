use crate::ast::{Binary, Expr, ExprVisitor, Grouping, Literal, Unary};

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&mut self, expr: &Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize(&mut self, name: &str, exprs: &[&Expr]) -> String {
        let mut result = String::new();
        result.push('(');
        result.push_str(name);

        for expr in exprs {
            result.push(' ');
            result.push_str(&self.print(expr));
        }

        result.push(')');
        result
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary(&mut self, expr: &Binary) -> String {
        self.parenthesize(&expr.operator.lexeme, &[&expr.left, &expr.right])
    }

    fn visit_grouping(&mut self, expr: &Grouping) -> String {
        self.parenthesize("group", &[&expr.expression])
    }

    fn visit_literal(&mut self, expr: &Literal) -> String {
        match expr {
            Literal::Number(n) => n.to_string(),
            Literal::String(s) => s.clone(),
            Literal::Bool(b) => b.to_string(),
            Literal::Nil => "nil".to_string(),
        }
    }

    fn visit_unary(&mut self, expr: &Unary) -> String {
        self.parenthesize(&expr.operator.lexeme, &[&expr.right])
    }
}
