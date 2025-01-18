use crate::token::{LiteralValue, Token};

trait ExprTrait {
    fn accept<V: Visitor>(&self, visitor: &V) -> String;
}

#[derive(Debug)]
enum Expr {
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
struct BinaryExpr {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}
#[derive(Debug)]
struct GroupingExpr {
    expr: Box<Expr>,
}
#[derive(Debug)]
struct LiteralExpr {
    value: Option<LiteralValue>,
}
#[derive(Debug)]
struct UnaryExpr {
    operator: Token,
    right: Box<Expr>,
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
