use crate::token::Token;
use crate::token_type::Literal;

#[derive(Debug, Clone)]
pub enum Expr<'a> {
    Assign {
        name: &'a Token<'a>,
        value: Box<Expr<'a>>,
    },
    Binary {
        left: Box<Expr<'a>>,
        operator: &'a Token<'a>,
        right: Box<Expr<'a>>,
    },
    Call {
        callee: Box<Expr<'a>>,
        paren: &'a Token<'a>,
        arguments: Vec<Expr<'a>>,
    },
    Get {
        object: Box<Expr<'a>>,
        name: &'a Token<'a>,
    },
    Grouping {
        expression: Box<Expr<'a>>,
    },
    Literal {
        value: Literal<'a>,
    },
    Logical {
        left: Box<Expr<'a>>,
        operator: &'a Token<'a>,
        right: Box<Expr<'a>>,
    },
    Set {
        object: Box<Expr<'a>>,
        name: &'a Token<'a>,
        value: Box<Expr<'a>>,
    },
    Super {
        keyword: &'a Token<'a>,
        method: &'a Token<'a>,
    },
    This {
        keyword: &'a Token<'a>,
    },
    Unary {
        operator: &'a Token<'a>,
        right: Box<Expr<'a>>,
    },
    Variable {
        name: &'a Token<'a>,
    },
}

impl<'a> std::fmt::Display for Expr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expr::Assign { name, value } => write!(f, "{} = {}", name.lexeme, value),
            Expr::Binary {
                left,
                operator,
                right,
            } => write!(f, "({} {} {})", left, operator.lexeme, right),
            Expr::Call {
                callee,
                paren: _,
                arguments,
            } => {
                write!(f, "{}(", callee)?;
                for arg in arguments {
                    write!(f, " {}", arg)?;
                }
                write!(f, ")")?;
                Ok(())
            }
            Expr::Get { object, name } => write!(f, "({}).{}", object, name),
            Expr::Grouping { expression } => write!(f, "({})", expression),
            Expr::Literal { value } => write!(f, "{}", value),
            Expr::Logical {
                left,
                operator,
                right,
            } => write!(f, "({} {} {})", left, operator.lexeme, right),
            Expr::Set {
                object,
                name,
                value,
            } => write!(f, "({}).{} = {}", object, name.lexeme, value),
            Expr::Super { keyword: _, method } => write!(f, "super.{}", method.lexeme),
            Expr::This { keyword: _ } => write!(f, "this"),
            Expr::Unary { operator, right } => write!(f, "({}{})", operator.lexeme, right),
            Expr::Variable { name } => write!(f, "{}", name),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Variable<'a> {
    pub name: &'a Token<'a>,
}

impl<'a> std::fmt::Display for Variable<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name.lexeme)
    }
}
