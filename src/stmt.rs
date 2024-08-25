use crate::expr::{self, Expr};
use crate::token::Token;

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

pub enum Stmt<'a> {
    Block(Block<'a>),
    Class(Class<'a>),
    Expression(Expression<'a>),
    Function(Function<'a>),
    If(If<'a>),
    Print(Print<'a>),
    Return(Return<'a>),
    Var(Var<'a>),
    While(While<'a>),
}

pub struct Block<'a> {
    pub statements: Vec<Stmt<'a>>,
}

pub struct Class<'a> {
    pub name: Token<'a>,
    pub super_class: expr::Variable<'a>,
    pub methods: Vec<Function<'a>>,
}

pub struct Expression<'a> {
    pub expression: Expr<'a>,
}

pub struct Function<'a> {
    pub name: Token<'a>,
    pub params: Vec<Token<'a>>,
    pub body: Vec<Stmt<'a>>,
}

pub struct If<'a> {
    pub condition: Expr<'a>,
    pub then_branch: Box<Stmt<'a>>,
    pub else_branch: Option<Box<Stmt<'a>>>,
}

pub struct Print<'a> {
    pub expression: Expr<'a>,
}

pub struct Return<'a> {
    pub keyword: Token<'a>,
    pub value: Option<Expr<'a>>,
}

pub struct Var<'a> {
    pub name: Token<'a>,
    pub initializer: Option<Expr<'a>>,
}

pub struct While<'a> {
    pub condition: Expr<'a>,
    pub body: Box<Stmt<'a>>,
}
