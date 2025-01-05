use crate::token::{LiteralValue, Token, TokenType};

#[derive(Debug)]
struct Scanner {
    pub source: Vec<char>,
    pub tokens: Vec<Token>,
    line: usize,
    current: usize,
    start: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            tokens: vec![],
            line: 1,
            current: 0,
            start: 0,
        }
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: String::default(),
            literal: None,
            line: self.line,
        });
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<LiteralValue>) {
        let lexeme = String::from_iter(&self.source[self.start..self.current]);
        self.tokens.push(Token {
            token_type,
            lexeme,
            literal,
            line: self.line,
        });
    }

    fn peek_match(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 > self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    fn scan_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            _ = self.advance();
        }
        let value = String::from_iter(&self.source[self.start + 1..self.current]);
        _ = self.advance();
        self.add_token(TokenType::String, Some(LiteralValue::Text(value)));
    }

    fn scan_number(&mut self) {
        while self.peek().is_ascii_digit() {
            _ = self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            _ = self.advance();
        }
        while self.peek().is_ascii_digit() {
            _ = self.advance();
        }
        let value: f64 = String::from_iter(&self.source[self.start..self.current])
            .parse()
            .unwrap();
        self.add_token(TokenType::Number, Some(LiteralValue::Number(value)));
    }

    fn scan_identifier(&mut self) {
        let mut c = self.peek();
        while c.is_alphabetic() || c == '_' {
            c = self.advance();
        }
        let text = String::from_iter(&self.source[self.start..self.current]);
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

    fn scan_token(&mut self) {
        let c = self.advance();
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
                    while self.peek() != '\n' && !self.is_at_end() {
                        _ = self.advance();
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
                } else if c.is_ascii_alphabetic() || c == '_' {
                    self.scan_identifier();
                } else {
                    unreachable!("TODO");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_scanner_instance() {
        let source = "var lang = \"lox\";";
        let scanner = Scanner::new(source.to_string());
        assert_eq!(scanner.line, 1);
        assert_eq!(scanner.current, 0);
        assert_eq!(scanner.start, 0);
        assert!(scanner.tokens.is_empty());
        assert_eq!(scanner.source, source.to_string());
    }
}
