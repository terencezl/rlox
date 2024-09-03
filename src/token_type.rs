#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum TokenType<'a> {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING(&'a str),
    NUMBER(f64),

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal<'a> {
    String(&'a str),
    Number(f64),
    True,
    False,
    Nil,
}

impl<'a> From<Literal<'a>> for TokenType<'a> {
    fn from(literal: Literal<'a>) -> Self {
        match literal {
            Literal::String(s) => TokenType::STRING(s),
            Literal::Number(l) => TokenType::NUMBER(l),
            Literal::True => TokenType::TRUE,
            Literal::False => TokenType::FALSE,
            Literal::Nil => TokenType::NIL,
        }
    }
}

impl<'a> std::fmt::Display for Literal<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Literal::String(s) => write!(f, "{}", s),
            Literal::Number(n) => write!(f, "{}", n),
            Literal::True => write!(f, "true"),
            Literal::False => write!(f, "false"),
            Literal::Nil => write!(f, "nil"),
        }
    }
}
