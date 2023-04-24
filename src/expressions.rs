use crate::token::Token;

#[derive(Debug)]
pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
}

#[derive(Debug)]
pub struct BinaryExpr {
    left: Box<Expr>,
    right: Box<Expr>,
    operator: Token,
}

#[derive(Debug)]
pub struct GroupingExpr {
    expression: Box<Expr>,
}

#[derive(Debug)]
pub struct LiteralExpr {
    value: Token,
}

#[derive(Debug)]
pub struct UnaryExpr {
    operator: Token,
    right: Box<Expr>,
}

impl BinaryExpr {
    pub fn new(left: Box<Expr>, right: Box<Expr>, operator: Token) -> Self {
        Self {
            left,
            right,
            operator,
        }
    }
}

impl GroupingExpr {
    pub fn new(expression: Box<Expr>) -> Self {
        Self { expression }
    }
}

impl LiteralExpr {
    pub fn new(value: Token) -> Self {
        Self { value }
    }
}

impl UnaryExpr {
    pub fn new(operator: Token, right: Box<Expr>) -> Self {
        Self { operator, right }
    }
}
