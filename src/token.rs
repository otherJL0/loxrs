use std::fmt;

#[derive(Clone, Debug)]
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
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

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

    Eof,
}

#[derive(Clone, Debug)]
pub enum LiteralValue<'a> {
    Text(&'a str),
    Number(f64),
    Bool(bool),
}

#[derive(Clone, Debug)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub lexeme: &'a str,
    pub literal: Option<LiteralValue<'a>>,
    pub line: usize,
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} {} {:?}",
            self.token_type, self.lexeme, self.literal
        )
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
                    lexeme: actual.1,
                    literal: Some(LiteralValue::Text(actual.1)),
                    line: 1
                }
                .to_string(),
                expected.to_string()
            );
        }
    }
}
