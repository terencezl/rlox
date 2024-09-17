use crate::expr::Expr;
use crate::token_type::{Literal, TokenType};
use thiserror::Error;

#[derive(Error, Debug)]
#[error("{0}")]
pub struct RuntimeError(String);

pub type InterpreterResult<T> = Result<T, RuntimeError>;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Number(f64),
    Bool(bool),
    Nil,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::String(s) => write!(f, "{}", s),
            Value::Number(n) => write!(f, "{}", n),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
        }
    }
}

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret<'a>(&self, expr: &Expr<'a>) -> InterpreterResult<Value> {
        match expr {
            Expr::Literal { value } => match value {
                Literal::String(s) => Ok(Value::String(s.to_string())),
                Literal::Number(n) => Ok(Value::Number(*n)),
                Literal::True => Ok(Value::Bool(true)),
                Literal::False => Ok(Value::Bool(false)),
                Literal::Nil => Ok(Value::Nil),
            },
            Expr::Grouping { expression } => self.interpret(expression),
            Expr::Unary { operator, right } => {
                let right = self.interpret(right)?;
                match operator.typ {
                    TokenType::MINUS => match right {
                        Value::Number(n) => Ok(Value::Number(-n)),
                        _ => Err(RuntimeError("Operand must be a number".to_string())),
                    },
                    TokenType::BANG => match right {
                        Value::Bool(b) => Ok(Value::Bool(!b)),
                        _ => Err(RuntimeError("Operand must be a boolean".to_string())),
                    },
                    _ => Err(RuntimeError(format!(
                        "Unknown unary operator: {}",
                        operator
                    ))),
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.interpret(left)?;
                let right = self.interpret(right)?;
                match operator.typ {
                    TokenType::PLUS => match (left, right) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
                        (Value::String(l), Value::String(r)) => {
                            Ok(Value::String(format!("{}{}", l, r)))
                        }
                        _ => Err(RuntimeError(
                            "Operands must be two numbers or two strings".to_string(),
                        )),
                    },
                    TokenType::MINUS => match (left, right) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),
                        _ => Err(RuntimeError("Operands must be numbers".to_string())),
                    },
                    TokenType::STAR => match (left, right) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),
                        _ => Err(RuntimeError("Operands must be numbers".to_string())),
                    },
                    TokenType::SLASH => match (left, right) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l / r)),
                        _ => Err(RuntimeError("Operands must be numbers".to_string())),
                    },
                    TokenType::GREATER => match (left, right) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l > r)),
                        _ => Err(RuntimeError("Operands must be numbers".to_string())),
                    },
                    TokenType::GREATER_EQUAL => match (left, right) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l >= r)),
                        _ => Err(RuntimeError("Operands must be numbers".to_string())),
                    },
                    TokenType::LESS => match (left, right) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l < r)),
                        _ => Err(RuntimeError("Operands must be numbers".to_string())),
                    },
                    TokenType::LESS_EQUAL => match (left, right) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l <= r)),
                        _ => Err(RuntimeError("Operands must be numbers".to_string())),
                    },
                    TokenType::BANG_EQUAL => Ok(Value::Bool(left != right)),
                    TokenType::EQUAL_EQUAL => Ok(Value::Bool(left == right)),
                    _ => Err(RuntimeError(format!(
                        "Unknown binary operator: {}",
                        operator
                    ))),
                }
            }
            _ => Err(RuntimeError(format!("Unknown expression: {}", expr))),
        }
    }
}
