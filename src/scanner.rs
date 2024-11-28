use crate::token::Token;

#[allow(dead_code)]
struct Scanner {
    source: String,
    tokens: Vec<Token>,
    line: usize,
    current: usize,
    start: usize,
}

impl Scanner {
    fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: vec![],
            line: 1,
            current: 0,
            start: 0,
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
