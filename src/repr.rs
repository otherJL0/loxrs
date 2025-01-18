use crate::token::{LiteralValue, Token};

trait ExprTrait {
    fn accept<V: Visitor>(&self, visitor: &V) -> String;
}

#[derive(Debug)]
enum Expr<'a> {
    Binary(BinaryExpr<'a>),
    Grouping(GroupingExpr<'a>),
    Literal(LiteralExpr<'a>),
    Unary(UnaryExpr<'a>),
}

impl ExprTrait for Expr<'_> {
    fn accept<V: Visitor>(&self, visitor: &V) -> String {
        visitor.visit(self)
    }
}

#[derive(Debug)]
struct BinaryExpr<'a> {
    left: Box<Expr<'a>>,
    operator: Token<'a>,
    right: Box<Expr<'a>>,
}

impl<'a> BinaryExpr<'a> {
    pub fn new(left: Expr<'a>, operator: Token<'a>, right: Expr<'a>) -> BinaryExpr<'a> {
        BinaryExpr {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

#[derive(Debug)]
struct GroupingExpr<'a> {
    expr: Box<Expr<'a>>,
}

impl<'a> GroupingExpr<'a> {
    pub fn new(expr: Expr<'a>) -> GroupingExpr<'a> {
        GroupingExpr {
            expr: Box::new(expr),
        }
    }
}
#[derive(Debug)]
struct LiteralExpr<'a> {
    value: Option<LiteralValue<'a>>,
}

impl<'a> LiteralExpr<'a> {
    pub fn new(value: Option<LiteralValue<'a>>) -> LiteralExpr<'a> {
        LiteralExpr { value }
    }
}
#[derive(Debug)]
struct UnaryExpr<'a> {
    operator: Token<'a>,
    right: Box<Expr<'a>>,
}

impl<'a> UnaryExpr<'a> {
    pub fn new(operator: Token<'a>, right: Expr<'a>) -> UnaryExpr<'a> {
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
