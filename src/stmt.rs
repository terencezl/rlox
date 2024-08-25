use crate::token::Token;
use crate::expr::{self, Expr};

pub trait Visitor<T> {
    fn visit_block_stmt(&mut self, block: &Block) -> T;
    fn visit_class_stmt(&mut self, class: &Class) -> T;
    fn visit_expression_stmt(&mut self, expression: &Expression) -> T;
    fn visit_function_stmt(&mut self, function: &Function) -> T;
    fn visit_if_stmt(&mut self, if_: &If) -> T;
    fn visit_print_stmt(&mut self, print: &Print) -> T;
    fn visit_return_stmt(&mut self, return_: &Return) -> T;
    fn visit_var_stmt(&mut self, var: &Var) -> T;
    fn visit_while_stmt(&mut self, while_: &While) -> T;
}

pub enum Stmt {
    Block(Block),
    Class(Class),
    Expression(Expression),
    Function(Function),
    If(If),
    Print(Print),
    Return(Return),
    Var(Var),
    While(While),
}


pub struct Block {
    pub statements: Vec<Stmt>,
}

pub struct Class {
    pub name: Token,
    pub super_class: expr::Variable,
    pub methods: Vec<Function>,
}

pub struct Expression {
    pub expression: Expr,
}

pub struct Function {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}

pub struct If {
    pub condition: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

pub struct Print {
    pub expression: Expr,
}

pub struct Return {
    pub keyword: Token,
    pub value: Option<Expr>,
}

pub struct Var {
    pub name: Token,
    pub initializer: Option<Expr>,
}

pub struct While {
    pub condition: Expr,
    pub body: Box<Stmt>,
}
