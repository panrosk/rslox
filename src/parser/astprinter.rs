use core::panic;

use either::Either::{Left, Right};

use crate::{
    parser::expressions::Visitor,
    scanner::tokentype::{Literal, TokenType},
};

pub struct Astprinter {}

impl Visitor for Astprinter {
    fn visit_literalexpr(&self, expr: &super::expressions::LiteralExpr) -> String {
        match &expr.value {
            Left(c) => match c {
                Literal::Str(s) => s.clone(),
                Literal::Number(s) => s.clone().to_string(),
                Literal::Identifier(s) => s.clone().to_string(),
            },
            Right(c) => c.to_string(),
        }
    }

    fn visit_gropingexpr(&self, expr: &super::expressions::GroupingExpr) -> String {
        todo!()
    }

    fn visit_unaryexpr(&self, expr: &super::expressions::UnaryExpr) -> String {
        let operation = expr.operator;
    }

    fn visit_binaryexpr(&self, expr: &super::expressions::BinaryExpr) -> String {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::{
        parser::expressions::{LiteralExpr, Visitor},
        scanner::tokentype::{Literal, TokenType},
    };

    use super::Astprinter;

    #[test]
    fn astprinter_literal_left() {
        let literal = Literal::Str("hola".to_string());
        let literal_expresion = LiteralExpr {
            value: either::Either::Left(literal),
        };

        let ast_printer = Astprinter {};

        let val = ast_printer.visit_literalexpr(&literal_expresion);

        assert_eq!(val, "hola".to_string())
    }

    #[test]
    fn astprinter_literal_right() {
        let literal = TokenType::False;
        let literal_expresion = LiteralExpr {
            value: either::Either::Right(literal),
        };

        let ast_printer = Astprinter {};

        let val = ast_printer.visit_literalexpr(&literal_expresion);

        println!("{}", val);
        assert_eq!(val, "False".to_string())
    }
}
