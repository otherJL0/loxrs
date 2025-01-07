use crate::{
    repr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr},
    token::{LiteralValue, Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    idx: usize,
}

impl Parser {
    #[must_use]
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, idx: 0 }
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().token_type, TokenType::Eof)
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.idx - 1]
    }

    fn current(&self) -> &Token {
        &self.tokens[self.idx]
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.idx + 1]
    }

    fn advance(&mut self) -> &Token {
        self.idx += 1;
        self.previous()
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn primary(&mut self) -> Expr {
        match self.peek().token_type {
            TokenType::False => Expr::Literal(LiteralExpr::new(Some(LiteralValue::Bool(false)))),
            TokenType::True => Expr::Literal(LiteralExpr::new(Some(LiteralValue::Bool(true)))),
            TokenType::Nil => Expr::Literal(LiteralExpr::new(None)),
            TokenType::Number | TokenType::String => {
                Expr::Literal(LiteralExpr::new(self.previous().literal.clone()))
            }
            TokenType::LeftParen => {
                let expr = self.expression();
                if matches!(self.peek().token_type, TokenType::RightParen) {
                    _ = self.advance();
                }
                Expr::Grouping(GroupingExpr::new(Box::new(expr)))
            }
            _ => unreachable!("All other cases should be handled outside of primary"),
        }
    }

    fn unary(&mut self) -> Expr {
        match self.peek().token_type {
            TokenType::Bang | TokenType::Minus => {
                let operator = self.previous().clone();
                let right = self.unary();
                Expr::Unary(UnaryExpr::new(operator, Box::new(right)))
            }
            _ => self.primary(),
        }
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        loop {
            if matches!(self.peek().token_type, TokenType::Slash | TokenType::Star) {
                let operator = self.previous().clone();
                let right = self.unary();
                expr = Expr::Binary(BinaryExpr::new(Box::new(expr), operator, Box::new(right)));
            } else {
                break;
            }
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        loop {
            if matches!(self.peek().token_type, TokenType::Minus | TokenType::Plus) {
                let operator = self.previous().clone();
                let right = self.factor();
                expr = Expr::Binary(BinaryExpr::new(Box::new(expr), operator, Box::new(right)));
            } else {
                break;
            }
        }
        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        loop {
            if matches!(
                self.peek().token_type,
                TokenType::Greater
                    | TokenType::GreaterEqual
                    | TokenType::Less
                    | TokenType::LessEqual
            ) {
                let operator = self.previous().clone();
                let right = self.factor();
                expr = Expr::Binary(BinaryExpr::new(Box::new(expr), operator, Box::new(right)));
            } else {
                break;
            }
        }
        expr
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        loop {
            if matches!(
                self.peek().token_type,
                TokenType::BangEqual | TokenType::EqualEqual
            ) {
                let operator = self.previous().clone();
                let right = self.factor();
                expr = Expr::Binary(BinaryExpr::new(Box::new(expr), operator, Box::new(right)));
            } else {
                break;
            }
        }
        expr
    }
    pub fn parse(&mut self) -> Expr {
        self.expression()
    }
}
