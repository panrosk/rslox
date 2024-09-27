use std::fmt;

use crate::scanner::tokentype::{Literal, TokenType};

#[derive(Clone)]
pub struct Token {
    pub ty: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
    pub col: u32,
}

impl Token {
    pub fn new(
        ty: TokenType,
        lexeme: String,
        literal: Option<Literal>,
        line: usize,
        col: u32,
    ) -> Self {
        Token {
            ty,
            lexeme,
            literal,
            line,
            col,
        }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Token {{ ty: {:?}, lexeme: \"{}\", literal: {:?}, line: {:?}, col: {:?}}}",
            self.ty, self.lexeme, self.literal, self.line, self.col
        )
    }
}
