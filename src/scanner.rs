use crate::token::Token;

pub struct Scanner<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan(&mut self) -> Vec<Token> {
        vec![]
    }
}
