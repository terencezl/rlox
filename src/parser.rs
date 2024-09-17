use crate::expr::Expr;
use crate::token::Token;
use crate::token_type::{Literal, TokenType};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError<'a> {
    #[error("Unexpected Token: Expected {0} on line {}, but got {}!", .1.line, .1.lexeme)]
    UnexpectedToken(&'static str, &'a Token<'a>),
    #[error("Unexpected EOF: {0}")]
    UnexpectedEOF(&'static str),
}

type ParserResult<'a> = Result<Expr<'a>, ParserError<'a>>;

pub struct Parser<'a> {
    tokens: &'a [Token<'a>],
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token<'a>]) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(mut self) -> ParserResult<'a> {
        self.expression()
    }

    fn expression(&mut self) -> ParserResult<'a> {
        self.equality()
    }

    fn equality(&mut self) -> ParserResult<'a> {
        let mut expr = self.comparison()?;
        while let Some(t) = self
            .peek()
            .filter(|t| matches!(t.typ, TokenType::BANG_EQUAL | TokenType::EQUAL_EQUAL))
        {
            self.advance();
            let operator = t;
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> ParserResult<'a> {
        let mut expr = self.term()?;
        while let Some(t) = self.peek().filter(|t| {
            matches!(
                t.typ,
                TokenType::GREATER
                    | TokenType::GREATER_EQUAL
                    | TokenType::LESS
                    | TokenType::LESS_EQUAL
            )
        }) {
            let operator = t;
            self.advance();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn term(&mut self) -> ParserResult<'a> {
        let mut expr = self.factor()?;
        while let Some(t) = self
            .peek()
            .filter(|t| matches!(t.typ, TokenType::MINUS | TokenType::PLUS))
        {
            self.advance();
            let operator = t;
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn factor(&mut self) -> ParserResult<'a> {
        let mut expr = self.unary()?;
        while let Some(t) = self
            .peek()
            .filter(|t| matches!(t.typ, TokenType::SLASH | TokenType::STAR))
        {
            self.advance();
            let operator = t;
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn unary(&mut self) -> ParserResult<'a> {
        if let Some(t) = self
            .peek()
            .filter(|t| matches!(t.typ, TokenType::BANG | TokenType::MINUS))
        {
            self.advance();
            let operator = t;
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }
        self.primary()
    }

    fn primary(&mut self) -> ParserResult<'a> {
        if let Some(t) = self.peek() {
            match t.typ {
                TokenType::FALSE => {
                    self.advance();
                    Ok(Expr::Literal {
                        value: Literal::False,
                    })
                }
                TokenType::TRUE => {
                    self.advance();
                    Ok(Expr::Literal {
                        value: Literal::True,
                    })
                }
                TokenType::NIL => {
                    self.advance();
                    Ok(Expr::Literal {
                        value: Literal::Nil,
                    })
                }
                TokenType::NUMBER(n) => {
                    self.advance();
                    Ok(Expr::Literal {
                        value: Literal::Number(n),
                    })
                }
                TokenType::STRING(s) => {
                    self.advance();
                    Ok(Expr::Literal {
                        value: Literal::String(s),
                    })
                }
                TokenType::LEFT_PAREN => {
                    self.advance();
                    let expr = self.expression()?;
                    if let Some(t) = self.peek() {
                        if matches!(t.typ, TokenType::RIGHT_PAREN) {
                            self.advance();
                            Ok(Expr::Grouping {
                                expression: Box::new(expr),
                            })
                        } else {
                            Err(ParserError::UnexpectedToken(")", t))
                        }
                    } else {
                        Err(ParserError::UnexpectedEOF(
                            "Expected ')' after grouping expression but no token left!",
                        ))
                    }
                }
                _ => Err(ParserError::UnexpectedToken("literal", t)),
            }
        } else {
            Err(ParserError::UnexpectedEOF("No token left!"))
        }
    }

    fn peek(&self) -> Option<&'a Token<'a>> {
        let t = self.tokens.get(self.current);
        if t.is_some_and(|t| matches!(t.typ, TokenType::EOF)) {
            return None;
        }
        t
    }

    fn advance(&mut self) -> Option<&'a Token<'a>> {
        let c = self.tokens.get(self.current);
        self.current += 1;
        c
    }
}
