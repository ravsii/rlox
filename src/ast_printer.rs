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

// INFO: it's just a concept / homework.
// I don't wanna keep 2 printers up to date, so this one is left in its baby stage.
//
// pub struct AstPrinterRPN;
//
// impl AstPrinterRPN {
//     pub fn print(expr: &Expr) -> String {
//         match expr {
//             Expr::Binary(binary) => format!(
//                 "({} {} {})",
//                 AstPrinterRPN::print(&binary.left),
//                 AstPrinterRPN::print(&binary.right),
//                 binary.operator.lexeme
//             ),
//             Expr::Grouping(grouping) => {
//                 format!("({} group)", AstPrinterRPN::print(&grouping.expression))
//             }
//             Expr::Literal(literal) => match literal {
//                 Literal::Number(n) => n.to_string(),
//                 Literal::String(s) => s.clone(),
//                 Literal::Bool(b) => b.to_string(),
//                 Literal::Nil => "nil".to_string(),
//             },
//             Expr::Unary(unary) => format!(
//                 "({} {})",
//                 AstPrinterRPN::print(&unary.right),
//                 unary.operator.lexeme
//             ),
//         }
//     }
// }
