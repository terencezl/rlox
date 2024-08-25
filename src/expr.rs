use crate::token::{self, Token};

pub trait Visitor<T> {
    fn visit_assign_expr(&mut self, assign: &Assign) -> T;
    fn visit_binary_expr(&mut self, binary: &Binary) -> T;
    fn visit_call_expr(&mut self, call: &Call) -> T;
    fn visit_get_expr(&mut self, get: &Get) -> T;
    fn visit_grouping_expr(&mut self, grouping: &Grouping) -> T;
    fn visit_literal_expr(&mut self, literal: &Literal) -> T;
    fn visit_logical_expr(&mut self, logical: &Logical) -> T;
    fn visit_set_expr(&mut self, set: &Set) -> T;
    fn visit_super_expr(&mut self, super_: &Super) -> T;
    fn visit_this_expr(&mut self, this: &This) -> T;
    fn visit_unary_expr(&mut self, unary: &Unary) -> T;
    fn visit_variable_expr(&mut self, variable: &Variable) -> T;
}

pub enum Expr {
    Assign(Assign),
    Binary(Binary),
    Call(Call),
    Get(Get),
    Grouping(Grouping),
    Literal(Literal),
    Logical(Logical),
    Set(Set),
    Super(Super),
    This(This),
    Unary(Unary),
    Variable(Variable),
}

pub struct Assign {
    pub name: Token,
    pub value: Box<Expr>,
}

pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct Call {
    pub callee: Box<Expr>,
    pub paren: Token,
    pub arguments: Vec<Box<Expr>>,
}

pub struct Get {
    pub object: Box<Expr>,
    pub name: Token,
}

pub struct Grouping {
    pub expression: Box<Expr>,
}

pub struct Literal {
    pub value: token::Literal,
}

pub struct Logical {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct Set {
    pub object: Box<Expr>,
    pub name: Token,
    pub value: Box<Expr>,
}

pub struct Super {
    pub keyword: Token,
    pub method: Token,
}

pub struct This {
    pub keyword: Token,
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct Variable {
    pub name: Token,
}
