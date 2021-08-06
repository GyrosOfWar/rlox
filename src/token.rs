#[derive(Debug)]
pub enum Literal<'a> {
    String(&'a str),
    Number(f64),
}

#[derive(Debug)]
pub struct Token<'a> {
    pub ty: TokenType,
    pub lexeme: &'a str,
    pub literal: Option<Literal<'a>>,
    pub line: usize,
}

impl<'a> Token<'a> {
    pub fn new(ty: TokenType, lexeme: &'a str, literal: Option<Literal<'a>>, line: usize) -> Self {
        Self {
            ty,
            lexeme,
            literal,
            line,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[rustfmt::skip]
pub enum TokenType {
    // Single-character tokens.
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // One or two character tokens.
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals.
    Identifier, String, Number,

    // Keywords.
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    Eof
}
