use crate::token_type::TokenType;

#[derive(Debug, PartialEq, Clone)]
pub enum Literal<'a> {
    String(&'a str),
    Number(f64),
    Bool(bool),
    Nil,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token<'a> {
    r#type: TokenType,
    lexeme: &'a str,
    literal: Option<Literal<'a>>,
    line: usize,
}

impl<'a> Token<'a> {
    pub fn new(
        r#type: TokenType,
        lexeme: &'a str,
        literal: Option<Literal<'a>>,
        line: usize,
    ) -> Self {
        Self {
            r#type,
            lexeme,
            literal,
            line,
        }
    }
}

impl<'a> std::fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?} {} {:?}", self.r#type, self.lexeme, self.literal)
    }
}
