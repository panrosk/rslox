use crate::{
    parser::expressions::Visitor,
    scanner::tokentype::{Literal, TokenType},
};

use either::Either::{Left, Right};

pub struct Astprinter;

impl Visitor for Astprinter {
    fn visit_literalexpr(&mut self, expr: &super::expressions::LiteralExpr) -> String {
        match &expr.value {
            Left(c) => match c {
                Literal::Str(s) => s.clone(),
                Literal::Number(s) => s.to_string(),
                Literal::Identifier(s) => s.clone(),
            },
            Right(c) => format!("{:?}", c),
        }
    }

    fn visit_gropingexpr(&mut self, expr: &super::expressions::GroupingExpr) -> String {
        format!("(group {})", expr.expression.accept(self))
    }

    fn visit_unaryexpr(&mut self, expr: &super::expressions::UnaryExpr) -> String {
        let operation = expr.operator.to_token().to_string();
        let expression = expr.expression.accept(self); // Visita la expresión interna
        format!("({} {})", operation, expression)
    }

    fn visit_binaryexpr(&mut self, expr: &super::expressions::BinaryExpr) -> String {
        let left = expr.left.accept(self);
        let operator = expr.operator.to_string();
        let right = expr.right.accept(self);
        format!("({} {} {})", left, operator, right)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        parser::expressions::{BinaryExpr, LiteralExpr, Visitor},
        scanner::tokentype::{Literal, TokenType},
    };

    use super::Astprinter;

    #[test]
    fn astprinter_literal_left() {
        let literal = Literal::Str("hola".to_string());
        let literal_expresion = LiteralExpr {
            value: either::Either::Left(literal),
        };

        let mut ast_printer = Astprinter {};

        let val = ast_printer.visit_literalexpr(&literal_expresion);

        assert_eq!(val, "hola".to_string())
    }

    #[test]
    fn astprinter_literal_right() {
        let literal = TokenType::False;
        let literal_expresion = LiteralExpr {
            value: either::Either::Right(literal),
        };

        let mut ast_printer = Astprinter {};

        let val = ast_printer.visit_literalexpr(&literal_expresion);

        println!("{}", val);
        assert_eq!(val, "False".to_string())
    }

    #[test]
    fn astprinter_binary() {
        let litertal1 = Literal::Number(64 as f64);
        let literal1Expresion = LiteralExpr {
            value: either::Either::Left(litertal1),
        };
        let litertal2 = Literal::Number(32 as f64);
        let literal2Expresion = LiteralExpr {
            value: either::Either::Left(litertal2),
        };

        let binary_expr = BinaryExpr {
            left: Box::new(literal1Expresion),
            operator: TokenType::Minus,
            right: Box::new(literal2Expresion),
        };
        let mut ast_printer = Astprinter {};

        let val = ast_printer.visit_binaryexpr(&binary_expr);

        println!("{}", val);
        assert_eq!(val, "(64 Minus 32)".to_string())
    }
}
