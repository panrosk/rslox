use either::Either;

use crate::scanner::tokentype::TokenType;

pub trait Expresions {}

pub struct PrimaryExpresion {
    value: Either<TokenType, Box<dyn Expresions>>,
}

pub struct UnaryExpresion {
    //This sould be an UnaryOperator as an enum an offer a conversion from token as the ambigous
    //but whose got time for that.
    pub operator: Option<TokenType>,
    pub expresion: Box<dyn Expresions>,
}

pub struct FactorExpresion {
    pub left: UnaryExpresion,
    pub operator: TokenType,
    pub right: UnaryExpresion,
}

pub struct TermExpresion {
    pub left: FactorExpresion,
    pub operator: TokenType,
    pub right: FactorExpresion,
}

pub struct ComparsionExpresion {
    pub left: FactorExpresion,
    pub operator: TokenType,
}

impl Expresions for PrimaryExpresion {}
