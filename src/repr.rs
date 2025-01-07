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
    pub fn new(left: Box<Expr>, operator: Token, right: Box<Expr>) -> BinaryExpr {
        BinaryExpr {
            left,
            operator,
            right,
        }
    }
}
#[derive(Debug)]
pub struct GroupingExpr {
    expr: Box<Expr>,
}

impl GroupingExpr {
    pub fn new(expr: Box<Expr>) -> GroupingExpr {
        GroupingExpr { expr }
    }
}
#[derive(Debug)]
pub struct LiteralExpr {
    value: Option<LiteralValue>,
}

impl LiteralExpr {
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
    pub fn new(operator: Token, right: Box<Expr>) -> UnaryExpr {
        UnaryExpr { operator, right }
    }
}

trait Visitor {
    fn visit(&self, expr: &Expr) -> String;
}

#[derive(Default)]
pub struct AstPrinter {}

impl AstPrinter {
    #[must_use]
    pub fn print(&self, expr: &Expr) -> String {
        expr.accept(self)
    }
}

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
