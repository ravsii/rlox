use crate::{
    ast::{Binary, Expr, Literal, Stmt, Unary},
    environment::{Environment, EnvironmentError},
    token::{Token, TokenType},
};

pub struct InterpreterError {
    pub operator: Token,
    pub message: String,
}

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new(environment: Environment) -> Self {
        Interpreter { environment }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), InterpreterError> {
        for statement in statements {
            self.execute(statement)?;
        }

        Ok(())
    }

    pub fn execute(&mut self, statement: Stmt) -> Result<(), InterpreterError> {
        match statement {
            Stmt::Print(expr) => println!("{}", self.evaluate(expr)?),
            Stmt::Var(stmt) => {
                let value = self.evaluate(*stmt.initializer)?;
                self.environment.define(&stmt.name.lexeme, value);
            }
            Stmt::Nop => {}
        }

        Ok(())
    }

    pub fn evaluate(&mut self, expr: Expr) -> Result<Literal, InterpreterError> {
        match expr {
            Expr::Assign { name, value } => self.assign(name, *value),
            Expr::Binary(binary) => self.eval_binary(binary),
            Expr::Grouping(grouping) => self.evaluate(*grouping.expression),
            Expr::Literal(literal) => Ok(literal),
            Expr::Unary(unary) => self.eval_unary(unary),
            Expr::Variable(var) => match self.environment.get(&var.name.lexeme) {
                Ok(value) => Ok(value),
                Err(err) => match err {
                    EnvironmentError::UndefinedVariable(msg) => Err(InterpreterError {
                        operator: var.name,
                        message: msg,
                    }),
                },
            },
        }
    }

    fn assign(&mut self, name: Token, expr: Expr) -> Result<Literal, InterpreterError> {
        let value = self.evaluate(expr)?;
        match self.environment.assign(&name.lexeme, value.clone()) {
            Ok(value) => Ok(value),
            Err(err) => match err {
                EnvironmentError::UndefinedVariable(msg) => Err(InterpreterError {
                    operator: name,
                    message: msg,
                }),
            },
        }
    }

    fn eval_binary(&mut self, binary: Binary) -> Result<Literal, InterpreterError> {
        let left = self.evaluate(*binary.left)?;
        let right = self.evaluate(*binary.right)?;

        match binary.operator.token_type {
            TokenType::Minus => match (left, right) {
                (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Number(l - r)),
                _ => self.make_binary_err(binary.operator),
            },
            TokenType::Slash => match (left, right) {
                (Literal::Number(_), Literal::Number(0.0)) => Err(InterpreterError {
                    operator: binary.operator,
                    message: "Division by 0".into(),
                }),
                (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Number(l / r)),
                _ => self.make_binary_err(binary.operator),
            },
            TokenType::Star => match (left, right) {
                (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Number(l * r)),
                (Literal::Number(l), Literal::String(r)) => {
                    Ok(Literal::String(r.repeat(l as usize)))
                }
                (Literal::String(l), Literal::Number(r)) => {
                    Ok(Literal::String(l.repeat(r as usize)))
                }
                _ => self.make_binary_err(binary.operator),
            },
            TokenType::Plus => match (left, right) {
                (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Number(l + r)),
                (Literal::String(l), Literal::String(r)) => {
                    Ok(Literal::String(format!("{}{}", l, r)))
                }
                (Literal::String(l), Literal::Number(r)) => {
                    Ok(Literal::String(format!("{}{}", l, r)))
                }
                (Literal::Number(l), Literal::String(r)) => {
                    Ok(Literal::String(format!("{}{}", l, r)))
                }
                _ => Err(InterpreterError {
                    operator: binary.operator,
                    message: "Both operands must be numbers or strings.".into(),
                }),
            },
            TokenType::Greater => match (left, right) {
                (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Bool(l > r)),
                _ => self.make_binary_err(binary.operator),
            },
            TokenType::GreaterEqual => match (left, right) {
                (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Bool(l >= r)),
                _ => self.make_binary_err(binary.operator),
            },
            TokenType::Less => match (left, right) {
                (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Bool(l < r)),
                _ => self.make_binary_err(binary.operator),
            },
            TokenType::LessEqual => match (left, right) {
                (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Bool(l <= r)),
                _ => self.make_binary_err(binary.operator),
            },
            TokenType::BangEqual => Ok(Literal::Bool(!self.is_equal(left, right))),
            TokenType::EqualEqual => Ok(Literal::Bool(self.is_equal(left, right))),

            _ => Ok(Literal::Nil),
        }
    }

    fn make_binary_err(&self, operator: Token) -> Result<Literal, InterpreterError> {
        Err(InterpreterError {
            operator,
            message: "Both operands must be numbers.".into(),
        })
    }

    fn eval_unary(&mut self, unary: Unary) -> Result<Literal, InterpreterError> {
        let right = self.evaluate(*unary.right)?;

        if unary.operator.token_type == TokenType::Minus {
            return match right {
                Literal::Number(n) => Ok(Literal::Number(-n)),
                _ => self.make_unary_err(unary.operator),
            };
        }

        if unary.operator.token_type == TokenType::Bang {
            return Ok(Literal::Bool(!self.is_truthy(right)));
        }

        self.make_unary_err(unary.operator)
    }

    fn make_unary_err(&self, operator: Token) -> Result<Literal, InterpreterError> {
        Err(InterpreterError {
            operator: operator.clone(),
            message: "Operand must be a number.".into(),
        })
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
