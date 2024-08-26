use crate::token::{Literal, Token};
use crate::token_type::TokenType;
use crate::utils::take_slice;
use anyhow::Result;
use unicode_segmentation::UnicodeSegmentation;

pub struct Scanner<'a> {
    source: Vec<&'a str>,
    tokens: Vec<Token<'a>>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.graphemes(true).collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(mut self) -> Result<Vec<Token<'a>>> {
        while let Some(_) = self.scan_token() {}

        self.tokens
            .push(Token::new(TokenType::EOF, "", None, self.line));
        Ok(self.tokens)
    }

    fn scan_token(&mut self) -> Option<()> {
        let line = self.line;
        self.advance_while(|c| c == " " || c == "\t" || c == "\r" || c == "\n");

        self.start = self.current;
        let c = self.advance()?;
        match c {
            "(" => self.add_token(TokenType::LEFT_PAREN),
            ")" => self.add_token(TokenType::RIGHT_PAREN),
            "{" => self.add_token(TokenType::LEFT_BRACE),
            "}" => self.add_token(TokenType::RIGHT_BRACE),
            "," => self.add_token(TokenType::COMMA),
            "." => self.add_token(TokenType::DOT),
            "-" => self.add_token(TokenType::MINUS),
            "+" => self.add_token(TokenType::PLUS),
            ";" => self.add_token(TokenType::SEMICOLON),
            "*" => self.add_token(TokenType::STAR),
            "!" => {
                let token_type = if self.peek() == Some("=") {
                    // will never be None
                    self.advance();
                    TokenType::BANG_EQUAL
                } else {
                    TokenType::BANG
                };
                self.add_token(token_type);
            }
            "=" => {
                let token_type = if self.peek() == Some("=") {
                    // will never be None
                    self.advance();
                    TokenType::EQUAL_EQUAL
                } else {
                    TokenType::EQUAL
                };
                self.add_token(token_type);
            }
            "<" => {
                let token_type = if self.peek() == Some("=") {
                    // will never be None
                    self.advance();
                    TokenType::LESS_EQUAL
                } else {
                    TokenType::LESS
                };
                self.add_token(token_type);
            }
            ">" => {
                let token_type = if self.peek() == Some("=") {
                    // will never be None
                    self.advance();
                    TokenType::GREATER_EQUAL
                } else {
                    TokenType::GREATER
                };
                self.add_token(token_type);
            }
            "/" => {
                if self.peek() == Some("/") {
                    self.slash_slash_comment();
                } else if self.peek() == Some("*") {
                    self.slash_star_comment()?;
                } else {
                    self.add_token(TokenType::SLASH);
                }
            }
            "\"" => {
                self.string()?;
            }
            _ => {
                if is_digit(c) {
                    self.number();
                } else if is_alpha(c) {
                    self.identifier();
                } else {
                    error(line, format!("Unexpected character '{}'.", c).as_str());
                }
            }
        }
        Some(())
    }

    fn peek(&self) -> Option<&str> {
        self.source.get(self.current).map(|&c| c)
    }

    fn advance(&mut self) -> Option<&str> {
        let &c = self.source.get(self.current)?;
        if c == "\n" {
            self.line += 1;
        }

        self.current += 1;
        Some(c)
    }

    fn advance_while(&mut self, predicate: fn(&str) -> bool) {
        while self.peek().is_some_and(predicate) {
            // will never be None
            self.advance();
        }
    }

    fn peek_next(&self) -> Option<&str> {
        self.source.get(self.current + 1).map(|&c| c)
    }

    fn add_token(&mut self, r#type: TokenType) {
        self.add_token_with_literal(r#type, None);
    }

    fn add_token_with_literal(&mut self, r#type: TokenType, literal: Option<Literal<'a>>) {
        self.tokens.push(Token::new(
            r#type,
            take_slice(&self.source, self.start, self.current),
            literal,
            self.line,
        ));
    }

    fn string(&mut self) -> Option<()> {
        self.advance_while(|c| c != "\"");

        if self.peek() == None {
            error(self.line, "Unterminated string.");
            return None;
        }

        // consume closing "
        // will never be None
        self.advance();

        let literal = Literal::String(take_slice(&self.source, self.start + 1, self.current - 1));
        self.add_token_with_literal(TokenType::STRING, Some(literal));
        Some(())
    }

    fn slash_slash_comment(&mut self) {
        self.advance_while(|c| c != "\n");
    }

    fn slash_star_comment(&mut self) -> Option<()> {
        // consume the "*" of the opening "/*"
        // will never be None
        self.advance();

        while let Some(c) = self.peek() {
            // having self.peek_next() inside the loop ensures missing closing "/" is caught as error
            if c == "*" && self.peek_next() == Some("/") {
                break;
            }
            // will never be None
            self.advance();
        }

        if self.peek() == None {
            error(self.line, "Unterminated /* */ comment.");
            return None;
        }

        // consume closing "*/"
        // will never be None
        self.advance();
        // will never be None
        self.advance();

        Some(())
    }

    fn number(&mut self) {
        self.advance_while(is_digit);

        // look for a fractional part
        let next_is_digit = self.peek_next().is_some_and(is_digit);
        if self.peek() == Some(".") && next_is_digit {
            // consume the "."
            // will never be None
            self.advance();
            self.advance_while(is_digit);
        }

        let literal = Literal::Number(
            take_slice(&self.source, self.start, self.current)
                .parse::<f64>()
                .unwrap(),
        );
        self.add_token_with_literal(TokenType::NUMBER, Some(literal));
    }

    fn identifier(&mut self) {
        self.advance_while(is_alphanumeric);

        let text = take_slice(&self.source, self.start, self.current);
        let token_type = match text {
            "and" => TokenType::AND,
            "class" => TokenType::CLASS,
            "else" => TokenType::ELSE,
            "false" => TokenType::FALSE,
            "for" => TokenType::FOR,
            "fun" => TokenType::FUN,
            "if" => TokenType::IF,
            "nil" => TokenType::NIL,
            "or" => TokenType::OR,
            "print" => TokenType::PRINT,
            "return" => TokenType::RETURN,
            "super" => TokenType::SUPER,
            "this" => TokenType::THIS,
            "true" => TokenType::TRUE,
            "var" => TokenType::VAR,
            "while" => TokenType::WHILE,
            _ => TokenType::IDENTIFIER,
        };
        self.add_token(token_type);
    }
}

fn error(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, location: &str, message: &str) {
    eprintln!("[line {}] Error {}: {}", line, location, message);
}

fn is_digit(c: &str) -> bool {
    c.chars().all(|c| c.is_ascii_digit())
}

fn is_alpha(c: &str) -> bool {
    c.chars().all(|c| c.is_ascii_alphabetic() || c == '_')
}

fn is_alphanumeric(c: &str) -> bool {
    c.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}
