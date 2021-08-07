use crate::{
    expr::Expr,
    token::{Token, TokenType},
};

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Self { tokens, current: 0 }
    }

    fn at_end(&self) -> bool {
        self.peek().ty == TokenType::Eof
    }

    fn peek(&'a self) -> &'a Token<'a> {
        &self.tokens[self.current]
    }

    fn previous(&'a self) -> &'a Token<'a> {
        &self.tokens[self.current - 1]
    }

    fn advance(&'a mut self) -> &'a Token<'a> {
        if !self.at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn check(&self, ty: TokenType) -> bool {
        if self.at_end() {
            false
        } else {
            self.peek().ty == ty
        }
    }

    fn matches_any(&'a mut self, types: &[TokenType]) -> bool {
        for ty in types {
            if self.check(*ty) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn expression(&'a mut self) -> Expr<'a> {
        self.equality()
    }

    fn comparison(&mut self) -> Expr<'a> {
        todo!()
    }

    fn equality(&'a mut self) -> Expr<'a> {
        let mut expr = self.comparison();

        while self.matches_any(&[TokenType::EqualEqual, TokenType::BangEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::binary(expr, operator, right);
        }

        expr
    }
}
