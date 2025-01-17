use core::fmt;

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
    pub source: &'a str,
    pub tokens: Vec<Token<'a>>,
    line: usize,
    current: usize,
    start: usize,
}

impl<'a> Lexer<'a> {
    #[must_use]
    pub fn new(source: &'a str) -> Lexer<'a> {
        Lexer {
            source,
            tokens: vec![],
            line: 1,
            current: 0,
            start: 0,
        }
    }
    fn lookahead(&self, offset: usize) -> Option<char> {
        self.source[self.current + offset..].chars().next()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.lookahead(0)?;
        self.current += 1;
        Some(c)
    }

    /// # Errors
    ///
    /// Will return `Err` if an invalid character is detected
    pub fn scan_tokens(&mut self) -> Result<&[Token], InvalidCharacterError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }
        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "",
            literal: None,
            line: self.line,
        });
        Ok(&self.tokens)
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<LiteralValue<'a>>) {
        let lexeme = &self.source[self.start..self.current];
        self.tokens.push(Token {
            token_type,
            lexeme,
            literal,
            line: self.line,
        });
    }

    fn peek_match(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.lookahead(0).unwrap_or_default() != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn scan_string(&mut self) {
        while let Some(next_char) = self.lookahead(0)
            && next_char != '"'
        {
            if next_char == '\n' {
                self.line += 1;
            }
            self.current += 1;
        }
        let value = &self.source[self.start + 1..self.current];
        _ = self.advance();
        self.add_token(TokenType::String, Some(LiteralValue::Text(value)));
    }

    fn scan_number(&mut self) {
        while let Some(next_char) = self.lookahead(0)
            && next_char.is_ascii_digit()
        {
            self.current += 1;
        }
        if let Some(next_char) = self.lookahead(0) {
            if next_char == '.'
                && let Some(next_next_char) = self.lookahead(1)
                && next_next_char.is_ascii_digit()
            {
                self.current += 1;
            }
        }
        while let Some(next_char) = self.lookahead(0)
            && next_char.is_ascii_digit()
        {
            self.current += 1;
        }
        let value: f64 = self.source[self.start..self.current]
            .to_string()
            .parse()
            .unwrap();
        self.add_token(TokenType::Number, Some(LiteralValue::Number(value)));
    }

    fn scan_identifier(&mut self) {
        while let Some(next_char) = self.lookahead(0) {
            if !next_char.is_ascii_identifier_char() {
                break;
            }
            _ = self.advance();
        }
        let text = &self.source[self.start..self.current];
        let token_type = match text {
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
        if let Some(c) = self.advance() {
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
                        while let Some(next_char) = self.lookahead(0)
                            && next_char != '\n'
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
