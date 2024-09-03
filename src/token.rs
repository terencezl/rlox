use crate::token_type::TokenType;

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
    pub typ: TokenType<'a>,
    pub lexeme: &'a str,
    pub line: usize,
}

impl<'a> Token<'a> {
    pub fn new(typ: TokenType<'a>, lexeme: &'a str, line: usize) -> Self {
        Self { typ, lexeme, line }
    }
}

impl<'a> std::fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?} {}", self.typ, self.lexeme)
    }
}
