use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
};

use crate::{
    error,
    token::{self, *},
    token_type::*,
};

pub struct Scanner {
    source: String,
    tokens: RefCell<Vec<Token>>,
    start: u32,
    current: u32,
    line: u32,
    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let mut  keywords = HashMap::new();
        keywords.insert(String::from("and"), TokenType::AND);
        keywords.insert(String::from("class"), TokenType::CLASS);
        keywords.insert(String::from("else"), TokenType::ELSE);
        keywords.insert(String::from("false"), TokenType::FALSE);
        keywords.insert(String::from("for"), TokenType::FOR);
        keywords.insert(String::from("fun"), TokenType::FUN);
        keywords.insert(String::from("if"), TokenType::IF);
        keywords.insert(String::from("nil"), TokenType::NIL);
        keywords.insert(String::from("or"), TokenType::OR);
        keywords.insert(String::from("print"), TokenType::PRINT);
        keywords.insert(String::from("return"), TokenType::RETURN);
        keywords.insert(String::from("super"), TokenType::SUPER);
        keywords.insert(String::from("this"), TokenType::THIS);
        keywords.insert(String::from("true"), TokenType::TRUE);
        keywords.insert(String::from("var"), TokenType::VAR);
        keywords.insert(String::from("while"), TokenType::WHILE);

        Scanner {
            source,
            tokens: Vec::new().into(),
            start: 0,
            current: 0,
            line: 1,
            keywords,
        }
    }

    pub fn scan_tokens(&mut self) -> Ref<'_, Vec<token::Token>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.borrow_mut().push(Token::new(
            TokenType::EOF,
            String::new(),
            Literal::Nil,
            self.line,
        ));

        return self.tokens.borrow();
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();

        match c {
            '(' => self.add_token_nill(TokenType::LEFT_PAREN),
            ')' => self.add_token_nill(TokenType::RIGHT_PAREN),
            '{' => self.add_token_nill(TokenType::LEFT_BRACE),
            '}' => self.add_token_nill(TokenType::RIGHT_BRACE),
            ',' => self.add_token_nill(TokenType::COMMA),
            '.' => self.add_token_nill(TokenType::DOT),
            '-' => self.add_token_nill(TokenType::MINUS),
            '+' => self.add_token_nill(TokenType::PLUS),
            ';' => self.add_token_nill(TokenType::SEMICOLON),
            '*' => self.add_token_nill(TokenType::STAR),
            '!' => {
                let _type = if self.match_lexeme('=') {
                    TokenType::BANG_EQUAL
                } else {
                    TokenType::BANG
                };
                self.add_token_nill(_type);
            }
            '=' => {
                let _type = if self.match_lexeme('=') {
                    TokenType::EQUAL_EQUAL
                } else {
                    TokenType::EQUAL
                };
                self.add_token_nill(_type);
            }
            '<' => {
                let _type = if self.match_lexeme('=') {
                    TokenType::EQUAL_EQUAL
                } else {
                    TokenType::EQUAL
                };
                self.add_token_nill(_type);
            }
            '>' => {
                let _type = if self.match_lexeme('=') {
                    TokenType::GREATER
                } else {
                    TokenType::GREATER
                };

                self.add_token_nill(_type)
            }
            '/' => {
                if self.match_lexeme('/') {
                    // A comment goes until the end of the file.
                    while self.peek() != '\n' && self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token_nill(TokenType::SLASH);
                }
            }
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(),
            _ => {
                if self.is_digit(c) {
                    self.number()
                } else if self.is_alpha(c) {
                    self.identifier()
                } else {
                    error(self.line, "Unexpected character.")
                }
            }
        }
    }

    fn is_alpha(&self, c: char) -> bool {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        let result = self.is_alpha(c) || self.is_digit(c);
        return result;
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let start = self.start as usize;
        let current = self.current as usize;
        let text = self.source[start..current].to_string();

        let _type = self.keywords.get(&text);

        if let Some(_type) = _type  {
            
            self.add_token_nill(_type.clone())
        }
        
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        // Look for a fractional part
        if self.peek() == '.' && self.is_digit(self.peek()) {
            // Consume the .
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }
        let start = self.start as usize;
        let current = self.current as usize;
        let number: f64 = self.source[start..current].to_string().parse().unwrap();

        self.add_token_literal(TokenType::NUMBER, Literal::NumberLiteral(number))
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as u32
    }

    fn advance(&mut self) -> char {
        let current_char = self.source.chars().nth(self.current as usize);
        self.current += 1;

        match current_char {
            Some(c) => c,
            None => '\0',
        }
    }

    fn add_token_nill(&mut self, _type: TokenType) {
        self.add_token_literal(_type, Literal::Nil)
    }

    fn add_token_literal(&mut self, _type: TokenType, literal: Literal) {
        let text = self.source[self.start as usize..self.current as usize].to_string();
        self.tokens
            .borrow_mut()
            .push(Token::new(_type, text, literal, self.line))
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            error(self.line, "Unterminated string.");
            return;
        }

        // The closing ".
        self.advance();
        let start = self.start + 1;
        let current = self.current - 1;
        let value = self.source[start as usize..current as usize].to_string();
        self.add_token_literal(TokenType::STRING, Literal::StringLiteral(value));
    }

    fn match_lexeme(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        };

        if let Some(current) = self.source.chars().nth(self.current as usize) {
            if current != expected {
                return false;
            }
        }

        self.current += 1;
        false
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        };
        if let Some(current) = self.source.chars().nth(self.current as usize) {
            return current;
        } else {
            return '\0';
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() as u32 {
            return '\0';
        }

        if let Some(next_char) = self.source.chars().nth(self.current as usize) {
            next_char
        } else {
            return '\0';
        }
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }
}
