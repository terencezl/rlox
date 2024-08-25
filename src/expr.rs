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

pub enum Expr<'a> {
    Assign(Assign<'a>),
    Binary(Binary<'a>),
    Call(Call<'a>),
    Get(Get<'a>),
    Grouping(Grouping<'a>),
    Literal(Literal<'a>),
    Logical(Logical<'a>),
    Set(Set<'a>),
    Super(Super<'a>),
    This(This<'a>),
    Unary(Unary<'a>),
    Variable(Variable<'a>),
}

pub struct Assign<'a> {
    pub name: Token<'a>,
    pub value: Box<Expr<'a>>,
}

pub struct Binary<'a> {
    pub left: Box<Expr<'a>>,
    pub operator: Token<'a>,
    pub right: Box<Expr<'a>>,
}

pub struct Call<'a> {
    pub callee: Box<Expr<'a>>,
    pub paren: Token<'a>,
    pub arguments: Vec<Box<Expr<'a>>>,
}

pub struct Get<'a> {
    pub object: Box<Expr<'a>>,
    pub name: Token<'a>,
}

pub struct Grouping<'a> {
    pub expression: Box<Expr<'a>>,
}

pub struct Literal<'a> {
    pub value: token::Literal<'a>,
}

pub struct Logical<'a> {
    pub left: Box<Expr<'a>>,
    pub operator: Token<'a>,
    pub right: Box<Expr<'a>>,
}

pub struct Set<'a> {
    pub object: Box<Expr<'a>>,
    pub name: Token<'a>,
    pub value: Box<Expr<'a>>,
}

pub struct Super<'a> {
    pub keyword: Token<'a>,
    pub method: Token<'a>,
}

pub struct This<'a> {
    pub keyword: Token<'a>,
}

pub struct Unary<'a> {
    pub operator: Token<'a>,
    pub right: Box<Expr<'a>>,
}

pub struct Variable<'a> {
    pub name: Token<'a>,
}
