use crate::expr::{self, Expr};
use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Stmt<'a> {
    Block {
        statements: Vec<Stmt<'a>>,
    },
    Class {
        name: Token<'a>,
        super_class: expr::Variable<'a>,
        methods: Vec<Function<'a>>,
    },
    Expression {
        expression: Expr<'a>,
    },
    Function {
        name: Token<'a>,
        params: Vec<Token<'a>>,
        body: Vec<Stmt<'a>>,
    },
    If {
        condition: Expr<'a>,
        then_branch: Box<Stmt<'a>>,
        else_branch: Option<Box<Stmt<'a>>>,
    },
    Print {
        expression: Expr<'a>,
    },
    Return {
        keyword: Token<'a>,
        value: Option<Expr<'a>>,
    },
    Var {
        name: Token<'a>,
        initializer: Option<Expr<'a>>,
    },
    While {
        condition: Expr<'a>,
        body: Box<Stmt<'a>>,
    },
}

#[derive(Debug, Clone)]
pub struct Function<'a> {
    pub name: Token<'a>,
    pub params: Vec<Token<'a>>,
    pub body: Vec<Stmt<'a>>,
}
