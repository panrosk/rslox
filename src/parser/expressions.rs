// This was implemented in the book as a way to understand grammar. Problem is this grammar is
// ambigous. Im gonna leave at here with the AST printer wich use it and gona make a new one.

use std::fmt::Debug;

use either::Either;

use crate::scanner::tokentype::{Literal, TokenType};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum UnaryOperator {
    Minus,
    Bang,
}

impl UnaryOperator {
    pub fn from_token(token: TokenType) -> Option<UnaryOperator> {
        match token {
            TokenType::Minus => Some(UnaryOperator::Minus),
            TokenType::Bang => Some(UnaryOperator::Bang),
            _ => None,
        }
    }

    pub fn to_token(&self) -> TokenType {
        match self {
            UnaryOperator::Minus => TokenType::Minus,
            UnaryOperator::Bang => TokenType::Bang,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Star,
    Slash,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

impl Operator {
    pub fn from_token(token: TokenType) -> Option<Operator> {
        match token {
            TokenType::Plus => Some(Operator::Plus),
            TokenType::Minus => Some(Operator::Minus),
            TokenType::Star => Some(Operator::Star),
            TokenType::Slash => Some(Operator::Slash),
            TokenType::Bang => Some(Operator::Bang),
            TokenType::BangEqual => Some(Operator::BangEqual),
            TokenType::Equal => Some(Operator::Equal),
            TokenType::EqualEqual => Some(Operator::EqualEqual),
            TokenType::Greater => Some(Operator::Greater),
            TokenType::GreaterEqual => Some(Operator::GreaterEqual),
            TokenType::Less => Some(Operator::Less),
            TokenType::LessEqual => Some(Operator::LessEqual),
            _ => None, // No es un operador
        }
    }

    pub fn to_token(&self) -> TokenType {
        match self {
            Operator::Plus => TokenType::Plus,
            Operator::Minus => TokenType::Minus,
            Operator::Star => TokenType::Star,
            Operator::Slash => TokenType::Slash,
            Operator::Bang => TokenType::Bang,
            Operator::BangEqual => TokenType::BangEqual,
            Operator::Equal => TokenType::Equal,
            Operator::EqualEqual => TokenType::EqualEqual,
            Operator::Greater => TokenType::Greater,
            Operator::GreaterEqual => TokenType::GreaterEqual,
            Operator::Less => TokenType::Less,
            Operator::LessEqual => TokenType::LessEqual,
        }
    }
}

pub trait Expression: Debug {
    fn accept(&self, visitor: &mut dyn Visitor) -> String;
}

pub trait Visitor {
    fn visit_binaryexpr(&mut self, expr: &BinaryExpr) -> String;
    fn visit_unaryexpr(&mut self, expr: &UnaryExpr) -> String;
    fn visit_gropingexpr(&mut self, expr: &GroupingExpr) -> String;
    fn visit_literalexpr(&mut self, expr: &LiteralExpr) -> String;
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub left: Box<dyn Expression>,
    pub operator: TokenType,
    pub right: Box<dyn Expression>,
}

impl BinaryExpr {
    pub fn new(left: Box<dyn Expression>, operator: TokenType, right: Box<dyn Expression>) -> Self {
        BinaryExpr {
            left,
            right,
            operator,
        }
    }
}

impl Expression for BinaryExpr {
    fn accept(&self, visitor: &mut dyn Visitor) -> String {
        visitor.visit_binaryexpr(self)
    }
}

#[derive(Debug)]
pub struct UnaryExpr {
    pub operator: UnaryOperator,
    pub expression: Box<dyn Expression>,
}

impl Expression for UnaryExpr {
    fn accept(&self, visitor: &mut dyn Visitor) -> String {
        visitor.visit_unaryexpr(self)
    }
}

#[derive(Debug)]
pub struct GroupingExpr {
    pub expression: Box<dyn Expression>,
}

impl Expression for GroupingExpr {
    fn accept(&self, visitor: &mut dyn Visitor) -> String {
        visitor.visit_gropingexpr(self)
    }
}

#[derive(Debug)]
pub struct LiteralExpr {
    pub value: Either<Literal, TokenType>,
}

impl LiteralExpr {
    pub fn new(literal: Either<Literal, TokenType>) -> Self {
        LiteralExpr { value: literal }
    }
}

impl Expression for LiteralExpr {
    fn accept(&self, visitor: &mut dyn Visitor) -> String {
        visitor.visit_literalexpr(self)
    }
}
