use super::{
    token::Token,
    tokentype::{Literal, TokenType},
};
use core::panic;
use std::collections::HashMap;
use std::{char, str::Chars, u32, usize};

#[derive(Debug)]
struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: u32,
    current: u32,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn keywords() -> HashMap<String, TokenType> {
        let mut keywords = HashMap::new();
        keywords.insert("and".to_string(), TokenType::And);
        keywords.insert("class".to_string(), TokenType::Class);
        keywords.insert("else".to_string(), TokenType::Else);
        keywords.insert("false".to_string(), TokenType::False);
        keywords.insert("for".to_string(), TokenType::For);
        keywords.insert("fun".to_string(), TokenType::Fun);
        keywords.insert("if".to_string(), TokenType::If);
        keywords.insert("nil".to_string(), TokenType::Nil);
        keywords.insert("or".to_string(), TokenType::Or);
        keywords.insert("print".to_string(), TokenType::Print);
        keywords.insert("return".to_string(), TokenType::Return);
        keywords.insert("super".to_string(), TokenType::Super);
        keywords.insert("this".to_string(), TokenType::This);
        keywords.insert("true".to_string(), TokenType::True);
        keywords.insert("var".to_string(), TokenType::Var);
        keywords.insert("while".to_string(), TokenType::While);
        keywords
    }

    pub fn identifier(&mut self, start_position: u32) {
        while let Some(c) = self.source.chars().nth(self.current as usize) {
            if c.is_alphanumeric() || c == '_' {
                self.current += 1;
            } else {
                break;
            }
        }

        let text = &self.source[start_position as usize..self.current as usize];

        let token_type = Scanner::keywords()
            .get(text) // Try to find the keyword in the keywords map
            .cloned() // Clone the token type if found
            .unwrap_or(TokenType::Identifier); // Default to Identifier if not a keyword

        // Add the token (either a keyword or an identifier)
        self.add_token(token_type, text.to_string(), None);
    }

    pub fn number(&mut self, start_position: u32) {
        while let Some(c) = self.source.chars().nth(self.current as usize) {
            if !c.is_digit(10) {
                if c == '\n' {
                    panic!("You cant have multiline numbers")
                }
                break;
            }
            self.current += 1;
        }

        if self.current == self.source.len() as u32 {
            panic!("Hey you just did something funny, unnterminated number literal")
        } else {
            self.current -= 1;
            let number = &self.source[start_position as usize..(self.current + 1) as usize];
            self.add_token(
                TokenType::Number,
                "number".to_string(),
                Some(Literal::Number(
                    number.parse::<f64>().expect("This should wokr!"),
                )),
            );
        }
    }
    pub fn string(&mut self, start_position: u32) {
        self.current += 1;
        while let Some(c) = self.source.chars().nth(self.current as usize) {
            if c == '"' {
                break;
            }

            if c == '\n' {
                self.line += 1;
            }

            self.current += 1;
        }

        if self.current >= self.source.len() as u32 {
            panic!("Unterminated string literal");
        } else {
            self.current += 1;
            println!("{:?}", start_position);
            println!("{:?}", self.current);

            let literal_value =
                self.source[(start_position + 1) as usize..(self.current - 1) as usize].to_string();

            self.add_token(
                TokenType::String,
                "".to_string(),
                Some(Literal::Str(literal_value)),
            );
        }
    }

    pub fn match_char(&mut self, expected: char) -> bool {
        if self.current >= self.source.len() as u32 {
            return false;
        }
        if self.source.chars().nth((self.current + 1) as usize) != Some(expected) {
            return false;
        }
        self.current += 1;
        true
    }

    pub fn add_token(&mut self, token_type: TokenType, lexeme: String, literal: Option<Literal>) {
        let token_to_add = Token::new(token_type, lexeme, literal, self.line, self.start);
        self.tokens.push(token_to_add)
    }

    pub fn scan_tokens(&mut self) {
        while let Some(c) = self.source.chars().nth(self.current as usize) {
            println!("I read c{} in this {:?}", c, self.current);
            self.scan_token(c);
            self.current += 1;
            self.start += 1;
        }
    }

    pub fn scan_token(&mut self, c: char) {
        match c {
            '(' => self.add_token(TokenType::LeftParen, "(".to_string(), None),
            ')' => self.add_token(TokenType::RightParen, ")".to_string(), None),
            '{' => self.add_token(TokenType::LeftBrace, "{".to_string(), None),
            '}' => self.add_token(TokenType::RightBrace, "}".to_string(), None),
            ',' => self.add_token(TokenType::Comma, ",".to_string(), None),
            '.' => self.add_token(TokenType::Dot, ".".to_string(), None),
            '-' => self.add_token(TokenType::Minus, "-".to_string(), None),
            '+' => self.add_token(TokenType::Plus, "+".to_string(), None),
            ';' => self.add_token(TokenType::Semicolon, ";".to_string(), None),
            '*' => self.add_token(TokenType::Star, "*".to_string(), None),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual, "!=".to_string(), None);
                } else {
                    self.add_token(TokenType::Bang, "!".to_string(), None);
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual, "==".to_string(), None);
                } else {
                    self.add_token(TokenType::Equal, "=".to_string(), None);
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual, "<=".to_string(), None);
                } else {
                    self.add_token(TokenType::Less, "<".to_string(), None);
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual, ">=".to_string(), None);
                } else {
                    self.add_token(TokenType::Greater, ">".to_string(), None);
                }
            }
            '"' => self.string(self.current),

            '\n' => {
                self.line += 1;
                self.start = 0;
            }

            c if c.is_digit(10) => self.number(self.current),

            c if c.is_alphabetic() => self.identifier(self.current),

            // Ignore whitespace
            ' ' | '\r' | '\t' => (),

            _ => (),
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::scanner::token;

    use super::*;

    #[test]
    fn test_scanner_new() {
        let source_code = String::from("let x = 10;");
        let scanner = Scanner::new(source_code.clone());
        assert_eq!(scanner.source, source_code);
        assert!(scanner.tokens.is_empty());
    }

    #[test]
    fn test_scaner_scan() {
        let source_code = String::from("let x = (8;");
        let mut scanner = Scanner::new(source_code.clone());
        scanner.scan_tokens();
        assert!(!scanner.tokens.is_empty());
    }

    #[test]
    fn test_scanner_handles_newline() {
        let source_code = String::from("( \n )");
        let mut scanner = Scanner::new(source_code.clone());
        scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 2);

        assert_eq!(scanner.line, 2);
    }

    #[test]
    fn test_special_chars() {
        let source_code = String::from("( \n )");
        let mut scanner = Scanner::new(source_code.clone());
        scanner.scan_tokens();
    }

    #[test]
    fn test_scanner_handles_string_literal() {
        let source_code = String::from("\"Hola Mundo\"");
        let mut scanner = Scanner::new(source_code.clone());
        scanner.scan_tokens();
        assert_eq!(scanner.tokens.len(), 1);
    }

    #[test]
    fn test_scanner_handles_number_literal() {
        let source_code = String::from("1234 ");
        let mut scanner = Scanner::new(source_code.clone());
        scanner.scan_tokens();

        // Asegúrate de que se detecta correctamente un token de tipo número
        assert_eq!(scanner.tokens.len(), 1);

        // Verifica que el tipo de token sea `Number`

        // Asegúrate de que el literal sea el valor correcto
        if let Some(Literal::Number(value)) = &scanner.tokens[0].literal {
            assert_eq!(*value, 1234.0);
        }
    }

    // #[test]
    // #[should_panic(expected = "unnterminated number literal")]
    // fn test_scanner_handles_unterminated_number_literal() {
    //     let source_code = String::from("1234\n5678");
    //     let mut scanner = Scanner::new(source_code.clone());
    //     scanner.scan_tokens();
    // }
    //
    //

    #[test]
    fn test_identifier_and_keyword() {
        let source_code = String::from("var myVar = 10;");
        let mut scanner = Scanner::new(source_code.clone());
        scanner.scan_tokens();

        println!("{:?}", scanner.tokens);
        assert_eq!(scanner.tokens.len(), 5);

        assert_eq!(scanner.tokens[0].ty, TokenType::Var);

        assert_eq!(scanner.tokens[1].ty, TokenType::Identifier);

        assert_eq!(scanner.tokens[2].ty, TokenType::Equal);

        if let Some(Literal::Number(value)) = &scanner.tokens[3].literal {
            assert_eq!(*value, 10.0);
        }

        // Verify the last token is the semicolon ';'
        assert_eq!(scanner.tokens[4].ty, TokenType::Semicolon);
    }
}
