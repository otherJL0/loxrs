use core::fmt;
use itertools::{Itertools, MultiPeek};
use std::str::Chars;

use crate::{
    token::{LiteralValue, Token, TokenType},
    trait_extensions::IdentifierChar,
};

#[derive(Debug, Clone)]
pub struct InvalidCharacterError {
    invalid_char: char,
    column: usize,
    line: usize,
}

impl fmt::Display for InvalidCharacterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "invalid character detected: {} at line {}, column {}",
            self.invalid_char, self.line, self.column
        )
    }
}

#[derive(Debug)]
pub struct Lexer<'a> {
    pub source: MultiPeek<Chars<'a>>,
    pub tokens: Vec<Token>,
    line: usize,
    current: usize,
    start: usize,
}

impl<'a> Lexer<'a> {
    #[must_use]
    pub fn new(source: &'a str) -> Lexer<'a> {
        Lexer {
            source: source.chars().multipeek(),
            tokens: vec![],
            line: 1,
            current: 0,
            start: 0,
        }
    }

    /// # Errors
    ///
    /// Will return `Err` if an invalid character is detected
    pub fn scan_tokens(&mut self) -> Result<&[Token], InvalidCharacterError> {
        while self.source.peek().is_some() {
            self.start = self.current;
            self.scan_token()?;
        }
        self.tokens.push(Token {
            token_type: TokenType::Eof,
            literal: None,
            line: self.line,
        });
        Ok(&self.tokens)
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<LiteralValue>) {
        self.tokens.push(Token {
            token_type,
            literal,
            line: self.line,
        });
    }

    fn peek_match(&mut self, expected: char) -> bool {
        match self.source.peek() {
            None => false,
            Some(ch) => {
                if *ch != expected {
                    false
                } else {
                    self.source.reset_peek();
                    self.source.next();
                    true
                }
            }
        }
    }

    fn scan_string(&mut self) {
        let mut value = String::new();
        while let Some(next_char) = self.source.peek()
            && *next_char != '"'
        {
            if *next_char == '\n' {
                self.line += 1;
            }
            value.push(self.source.next().unwrap());
        }
        self.source.next();
        self.add_token(TokenType::String, Some(LiteralValue::Text(value)));
    }

    fn scan_number(&mut self) {
        let mut value = String::new();
        while let Some(next_char) = self.source.peek()
            && next_char.is_ascii_digit()
        {
            value.push(self.source.next().unwrap());
        }
        if let Some(next_char) = self.source.peek()
            && *next_char == '.'
            && let Some(next_next_char) = self.source.peek()
            && next_next_char.is_ascii_digit()
        {
            value.push(self.source.next().unwrap());
        }
        while let Some(next_char) = self.source.peek()
            && next_char.is_ascii_digit()
        {
            value.push(self.source.next().unwrap());
        }
        let value: f64 = value.parse().unwrap();
        self.add_token(TokenType::Number, Some(LiteralValue::Number(value)));
    }

    fn scan_identifier(&mut self) {
        let text = {
            let mut text = String::new();
            while let Some(next_char) = self.source.peek() {
                if !next_char.is_ascii_identifier_char() {
                    break;
                }
                text.push(self.source.next().unwrap());
            }
            text
        };
        let token_type = match text.as_str() {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "fun" => TokenType::Fun,
            "for" => TokenType::For,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        };
        self.add_token(token_type, Some(LiteralValue::Text(text)));
    }

    fn scan_token(&mut self) -> Result<(), InvalidCharacterError> {
        if let Some(c) = self.source.next() {
            match c {
                '(' => self.add_token(TokenType::LeftParen, None),
                ')' => self.add_token(TokenType::RightParen, None),
                '{' => self.add_token(TokenType::LeftBrace, None),
                '}' => self.add_token(TokenType::RightBrace, None),
                ',' => self.add_token(TokenType::Comma, None),
                '.' => self.add_token(TokenType::Dot, None),
                ';' => self.add_token(TokenType::SemiColon, None),
                '*' => {
                    let token_type = if self.peek_match('=') {
                        TokenType::StarEqual
                    } else {
                        TokenType::Star
                    };
                    self.add_token(token_type, None);
                }
                '+' => {
                    let token_type = if self.peek_match('=') {
                        TokenType::PlusEqual
                    } else {
                        TokenType::Plus
                    };
                    self.add_token(token_type, None);
                }
                '!' => {
                    let token_type = if self.peek_match('=') {
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    };
                    self.add_token(token_type, None);
                }
                '=' => {
                    let token_type = if self.peek_match('=') {
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    };
                    self.add_token(token_type, None);
                }
                '-' => {
                    let token_type = if self.peek_match('=') {
                        TokenType::MinusEqual
                    } else {
                        TokenType::Minus
                    };
                    self.add_token(token_type, None);
                }
                '<' => {
                    let token_type = if self.peek_match('=') {
                        TokenType::LessEqual
                    } else {
                        TokenType::Less
                    };
                    self.add_token(token_type, None);
                }
                '>' => {
                    let token_type = if self.peek_match('=') {
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Greater
                    };
                    self.add_token(token_type, None);
                }
                '/' => {
                    if self.peek_match('/') {
                        while let Some(next_char) = self.source.peek()
                            && *next_char != '\n'
                        {
                            self.current += 1;
                        }
                    } else {
                        let token_type = if self.peek_match('=') {
                            TokenType::SlashEqual
                        } else {
                            TokenType::Slash
                        };
                        self.add_token(token_type, None);
                    }
                }
                ' ' | '\r' | '\t' => {}
                '\n' => self.line += 1,
                '"' => self.scan_string(),
                _ => {
                    if c.is_ascii_digit() {
                        self.scan_number();
                    } else if c.is_ascii_identifier_char() {
                        self.scan_identifier();
                    } else {
                        return Err(InvalidCharacterError {
                            invalid_char: c,
                            column: self.start,
                            line: self.line,
                        });
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_lexer_instance() {
        let source = "var lang = \"lox\";";
        let lexer = Lexer::new(source);
        assert_eq!(lexer.line, 1);
        assert_eq!(lexer.current, 0);
        assert_eq!(lexer.start, 0);
        assert!(lexer.tokens.is_empty());
    }
}
