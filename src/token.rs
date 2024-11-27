use std::fmt;

#[derive(Debug)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    Comma,
    Dot,
    SemiColon,
    Colon,

    Plus,
    PlusEqual,
    Minus,
    MinusEqual,
    Equal,
    EqualEqual,
    Star,
    StarEqual,
    Slash,
    SlashEqual,

    Identifier,
    String,
    Number,

    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String,
    line: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {} {}", self.token_type, self.lexeme, self.literal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_display_format() {
        let test_cases = vec![
            ((TokenType::Dot, "."), "Dot . ."),
            ((TokenType::Plus, "+"), "Plus + +"),
            ((TokenType::Minus, "-"), "Minus - -"),
            ((TokenType::Star, "*"), "Star * *"),
            ((TokenType::Slash, "/"), "Slash / /"),
            ((TokenType::Equal, "="), "Equal = ="),
        ];
        for (actual, expected) in test_cases {
            assert_eq!(
                Token {
                    token_type: actual.0,
                    lexeme: actual.1.to_string(),
                    literal: actual.1.to_string(),
                    line: 1
                }
                .to_string(),
                expected.to_string()
            );
        }
    }
}
