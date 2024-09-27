use crate::scanner::token::Token;

struct Parser {
    current: u32,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        return Parser { current: 0, tokens };
    }
}
