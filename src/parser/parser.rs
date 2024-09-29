use super::expressions::{BinaryExpr, Expression, LiteralExpr, UnaryExpr, UnaryOperator};
use crate::scanner::{
    token::Token,
    tokentype::{Literal, TokenType},
};
use either::Either::{self, Right};
use std::borrow::Borrow;

#[derive(Debug)]
struct Parser {
    current: usize,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn match_token(&mut self, tokens_to_check: Vec<TokenType>) -> bool {
        for token in tokens_to_check.iter() {
            if self.check(token) {
                self.current += 1;
                return true;
            }
        }
        false
    }

    pub fn check(&mut self, token_type: &TokenType) -> bool {
        if *token_type == TokenType::Eof {
            return false;
        }
        if let Some(token) = self.tokens.get(self.current) {
            token.ty.borrow() == token_type
        } else {
            false
        }
    }

    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { current: 0, tokens }
    }

    pub fn expression(&mut self) -> Box<dyn Expression> {
        self.equality()
    }

    pub fn equality(&mut self) -> Box<dyn Expression> {
        let mut expr = self.comparison();

        while self.match_token(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.tokens[self.current - 1].clone().ty;

            let right = self.comparison();
            expr = Box::new(BinaryExpr::new(expr, operator, right));
        }
        expr
    }

    pub fn comparison(&mut self) -> Box<dyn Expression> {
        let mut expr = self.term();
        while self.match_token(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.tokens[self.current - 1].clone().ty;
            let right = self.term();
            expr = Box::new(BinaryExpr::new(expr, operator, right));
        }
        expr
    }

    pub fn term(&mut self) -> Box<dyn Expression> {
        let mut expr = self.factor();
        while self.match_token(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.tokens[self.current - 1].clone().ty;
            let right = self.factor();
            expr = Box::new(BinaryExpr::new(expr, operator, right));
        }
        expr
    }

    pub fn factor(&mut self) -> Box<dyn Expression> {
        let mut expr = self.unary();
        while self.match_token(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.tokens[self.current - 1].clone().ty;
            let right = self.unary();
            expr = Box::new(BinaryExpr::new(expr, operator, right));
        }
        expr
    }

    pub fn unary(&mut self) -> Box<dyn Expression> {
        if self.match_token(vec![TokenType::Minus, TokenType::Bang]) {
            let operator = self.tokens[self.current - 1].clone().ty;
            let right = self.unary();
            return Box::new(UnaryExpr {
                operator: UnaryOperator::from_token(operator).unwrap(),
                expression: right,
            });
        }
        self.primary()
    }

    pub fn primary(&mut self) -> Box<dyn Expression> {
        if self.match_token(vec![TokenType::False]) {
            return Box::new(LiteralExpr::new(Either::Right(TokenType::False)));
        }

        if self.match_token(vec![TokenType::True]) {
            return Box::new(LiteralExpr::new(Either::Right(TokenType::True)));
        }

        if self.match_token(vec![TokenType::Nil]) {
            return Box::new(LiteralExpr::new(Either::Right(TokenType::Nil)));
        }

        if self.match_token(vec![TokenType::Number, TokenType::String]) {
            return Box::new(LiteralExpr::new(Either::Left(
                self.tokens[self.current - 1].literal.clone().unwrap(),
            )));
        }

        panic!("Expected expression.")
    }

    pub fn parse(&mut self) -> Box<dyn Expression> {
        self.expression()
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use super::*;
    use crate::parser::astprinter::{self, Astprinter};
    use crate::parser::expressions::Visitor;
    use crate::scanner::token::Token;
    use crate::scanner::tokentype::TokenType;

    #[test]
    fn test_parser_equality_expression() {
        // Tokens representing the expression "4 == 4"
        let tokens = vec![
            Token {
                col: 1,
                line: 1,
                ty: TokenType::Number,
                lexeme: String::from("4"),
                literal: Some(Literal::Number(4.0)),
            },
            Token {
                col: 2,
                line: 1,
                ty: TokenType::EqualEqual,
                lexeme: String::from("=="),
                literal: None,
            },
            Token {
                col: 3,
                line: 1,
                ty: TokenType::Number,
                lexeme: String::from("4"),
                literal: Some(Literal::Number(4.0)),
            },
            Token {
                col: 4,
                line: 1,
                ty: TokenType::Eof,
                lexeme: String::from(""),
                literal: None,
            },
        ];

        let mut parser = Parser::new(tokens);
        let expression = parser.parse();

        println!("{:?}", expression.deref());
        assert_eq!("(4 == 4)", "(4 == 4)");
    }
}
