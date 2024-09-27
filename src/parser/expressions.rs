use either::Either;

use crate::scanner::tokentype::{Literal, TokenType};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum UnaryOperator {
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

    pub fn precedence(&self) -> u8 {
        match self {
            Operator::Plus | Operator::Minus => 1,
            Operator::Star | Operator::Slash => 2,
            Operator::BangEqual | Operator::EqualEqual => 3,
            Operator::Greater | Operator::GreaterEqual | Operator::Less | Operator::LessEqual => 4,
            Operator::Bang | Operator::Equal => 0,
        }
    }
}

pub trait Expression {
    fn accept(&self, visitor: &mut dyn Visitor) -> String;
}

pub trait Visitor {
    fn visit_binaryexpr(&self, expr: &BinaryExpr) -> String;
    fn visit_unaryexpr(&self, expr: &UnaryExpr) -> String;
    fn visit_gropingexpr(&self, expr: &GroupingExpr) -> String;
    fn visit_literalexpr(&self, expr: &LiteralExpr) -> String;
}

pub struct BinaryExpr {
    pub left: Box<dyn Expression>,
    pub operator: TokenType,
    pub right: Box<dyn Expression>,
}

impl Expression for BinaryExpr {
    fn accept(&self, visitor: &mut dyn Visitor) -> String {
        visitor.visit_binaryexpr(self)
    }
}

pub struct UnaryExpr {
    pub operator: UnaryOperator,
    pub expression: Box<dyn Expression>,
}

impl Expression for UnaryExpr {
    fn accept(&self, visitor: &mut dyn Visitor) -> String {
        visitor.visit_unaryexpr(self)
    }
}

pub struct GroupingExpr {
    pub expression: Box<dyn Expression>,
}

impl Expression for GroupingExpr {
    fn accept(&self, visitor: &mut dyn Visitor) -> String {
        visitor.visit_gropingexpr(self)
    }
}

pub struct LiteralExpr {
    pub value: Either<Literal, TokenType>,
}

impl Expression for LiteralExpr {
    fn accept(&self, visitor: &mut dyn Visitor) -> String {
        visitor.visit_literalexpr(self)
    }
}
