#[allow(dead_code)]
use crate::token::{LiteralValue, Token};

trait ExprTrait {
    fn accept<V: Visitor>(&self, visitor: &V) -> String;
}

#[derive(Debug)]
pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
}

impl ExprTrait for Expr {
    fn accept<V: Visitor>(&self, visitor: &V) -> String {
        visitor.visit(self)
    }
}

#[derive(Debug)]
pub struct BinaryExpr {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}

impl BinaryExpr {
    #[must_use]
    pub fn new(left: Expr, operator: Token, right: Expr) -> BinaryExpr {
        BinaryExpr {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

#[derive(Debug)]
pub struct GroupingExpr {
    expr: Box<Expr>,
}

impl GroupingExpr {
    #[must_use]
    pub fn new(expr: Expr) -> GroupingExpr {
        GroupingExpr {
            expr: Box::new(expr),
        }
    }
}
#[derive(Debug)]
pub struct LiteralExpr {
    value: Option<LiteralValue>,
}

impl LiteralExpr {
    #[must_use]
    pub fn new(value: Option<LiteralValue>) -> LiteralExpr {
        LiteralExpr { value }
    }
}
#[derive(Debug)]
pub struct UnaryExpr {
    operator: Token,
    right: Box<Expr>,
}

impl UnaryExpr {
    #[must_use]
    pub fn new(operator: Token, right: Expr) -> UnaryExpr {
        UnaryExpr {
            operator,
            right: Box::new(right),
        }
    }
}

trait Visitor {
    fn visit(&self, expr: &Expr) -> String;
}

struct AstPrinter {}

impl Visitor for AstPrinter {
    fn visit(&self, expr: &Expr) -> String {
        match expr {
            Expr::Binary(expr) => format!(
                "( {:?} {:?} {:?} )",
                expr.left.accept(self),
                expr.operator,
                expr.right.accept(self)
            )
            .to_string(),
            Expr::Grouping(expr) => format!("( {:?} )", expr.expr.accept(self)).to_string(),
            Expr::Literal(expr) => format!("( {:?} )", expr.value).to_string(),
            Expr::Unary(expr) => {
                format!("( {:?} {:?} )", expr.operator, expr.right.accept(self)).to_string()
            }
        }
    }
}
