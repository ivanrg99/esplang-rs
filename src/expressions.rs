use std::ops::Deref;
use crate::token::Token;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct BinaryExpr {
    left: Box<Expr>,
    right: Box<Expr>,
    operator: Token,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct GroupingExpr {
    expression: Box<Expr>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct LiteralExpr {
    value: Token,
}

#[allow(dead_code)]
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
    pub fn expression(&self) -> &Expr {
        self.expression.deref()
    }
}

impl LiteralExpr {
    pub fn new(value: Token) -> Self {
        Self { value }
    }
    pub fn value(&self) -> &Token {
        &self.value
    }
}

impl UnaryExpr {
    pub fn new(operator: Token, right: Box<Expr>) -> Self {
        Self { operator, right }
    }
    pub fn right(&self) -> &Expr {
        self.right.deref()
    }
    pub fn operator(&self) -> &Token {
        &self.operator
    }
}
