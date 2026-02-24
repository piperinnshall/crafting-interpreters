use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Token(pub TokenKind, pub String, pub usize);

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TokenKind: {}, lexeme: {}, line: {}", self.0, self.1, self.2)
    }
}


#[derive(Debug, Clone)]
pub enum TokenKind {
    // literals.
    Identifier(String), String(String), Number(f64),

    // Single-character tokens.
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // One or two character tokens.
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual, 
    Less, LessEqual,

    // keywords.
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    Eof,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Identifier(s) => write!(f, "Identifier({})", s),
            TokenKind::String(s) => write!(f, "String({})", s),
            TokenKind::Number(n) => write!(f, "Number({})", n),
            _ => write!(f, "{:?}", self),
        }
    }
}
