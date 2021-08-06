use crate::token::{Literal, Token, TokenType};
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        use TokenType::*;

        let mut map = HashMap::new();
        map.insert("and", And);
        map.insert("class", Class);
        map.insert("else", Else);
        map.insert("false", False);
        map.insert("for", For);
        map.insert("fun", Fun);
        map.insert("if", If);
        map.insert("nil", Nil);
        map.insert("or", Or);
        map.insert("print", Print);
        map.insert("return", Return);
        map.insert("super", Super);
        map.insert("this", This);
        map.insert("true", True);
        map.insert("var", Var);
        map.insert("while", While);
        map
    };
}

pub struct Scanner<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: usize,
    tokens: Vec<Token<'a>>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
            tokens: Vec::new(),
        }
    }

    fn at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> u8 {
        let byte = self.source.as_bytes()[self.current];
        self.current += 1;
        byte
    }

    fn peek(&self) -> u8 {
        if self.at_end() {
            b'\0'
        } else {
            self.source.as_bytes()[self.current]
        }
    }

    fn add_token(&mut self, ty: TokenType) {
        let text = &self.source[self.start..self.current];
        let token = Token::new(ty, text, None, self.line);
        self.tokens.push(token);
    }

    fn add_literal_token(&mut self, ty: TokenType, literal: Literal<'a>) {
        let text = &self.source[self.start..self.current];
        let token = Token::new(ty, text, Some(literal), self.line);
        self.tokens.push(token);
    }

    fn matches(&mut self, char: u8) -> bool {
        let c = self.peek();
        if self.at_end() {
            false
        } else if c != char {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn string(&mut self) {
        while self.peek() != b'"' && !self.at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.at_end() {
            log::error!("unterminated string");
            return;
        }

        // closing "
        self.advance();

        let text = &self.source[self.start + 1..self.current - 1];
        self.add_literal_token(TokenType::String, Literal::String(text));
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == b'.' && self.is_digit(self.peek_next()) {
            // eat the '.'
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let number: f64 = self.source[self.start..self.current]
            .parse()
            .expect("should be able to parse number");
        self.add_literal_token(TokenType::Number, Literal::Number(number));
    }

    fn peek_next(&self) -> u8 {
        if self.current + 1 >= self.source.len() {
            b'\0'
        } else {
            self.source.as_bytes()[self.current + 1]
        }
    }

    fn is_digit(&self, c: u8) -> bool {
        c.is_ascii_digit()
    }

    fn is_alpha(&self, c: u8) -> bool {
        c.is_ascii_alphabetic()
    }

    fn is_alphanumeric(&self, c: u8) -> bool {
        c.is_ascii_alphanumeric()
    }

    fn identifier(&mut self) {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }
        let text = &self.source[self.start..self.current];
        match KEYWORDS.get(&text) {
            Some(ty) => self.add_token(*ty),
            _ => self.add_token(TokenType::Identifier),
        }
    }

    fn scan_token(&mut self) {
        use TokenType::*;

        let c = self.advance();
        match c {
            b' ' | b'\t' | b'\r' => {}
            b'\n' => self.line += 1,
            b'(' => self.add_token(LeftParen),
            b')' => self.add_token(RightParen),
            b'{' => self.add_token(LeftBrace),
            b'}' => self.add_token(RightBrace),
            b',' => self.add_token(Comma),
            b'.' => self.add_token(Dot),
            b'-' => self.add_token(Minus),
            b'+' => self.add_token(Plus),
            b';' => self.add_token(Semicolon),
            b'*' => self.add_token(Star),
            b'!' => {
                let is_bang_eq = self.matches(b'=');
                self.add_token(if is_bang_eq { BangEqual } else { Bang });
            }
            b'=' => {
                let is_eq_eq = self.matches(b'=');
                self.add_token(if is_eq_eq { EqualEqual } else { Equal })
            }
            b'<' => {
                let is_lt_eq = self.matches(b'=');
                self.add_token(if is_lt_eq { LessEqual } else { Less });
            }
            b'>' => {
                let is_gt_eq = self.matches(b'=');
                self.add_token(if is_gt_eq { GreaterEqual } else { Greater });
            }
            b'/' => {
                let is_comment = self.matches(b'/');
                if is_comment {
                    while self.peek() != b'\n' && !self.at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(Slash);
                }
            }
            b'"' => {
                self.string();
            }
            _ => {
                if self.is_digit(c) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    log::error!(
                        "failed to tokenize, unexpected character at line {}: '{}' (ascii {})",
                        self.line,
                        c as char,
                        c
                    );

                    panic!("failed to tokenize");
                }
            }
        }
    }

    pub fn scan(mut self) -> Vec<Token<'a>> {
        while !self.at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens
            .push(Token::new(TokenType::Eof, "", None, self.line));
        self.tokens
    }
}
