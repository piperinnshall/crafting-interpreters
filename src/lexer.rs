use crate::token::{Token, TokenKind};

#[derive(Default, Clone)]
pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: i32,
    current: i32,
    line: i32,
}

impl Lexer {
    pub fn from(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            ..Default::default()
        }
    }

    pub fn advance_start(&mut self) {
        self.start = self.current
    }

    pub fn advance_line(&mut self) {
        self.line += 1
    }

    pub fn peek(&self) -> Option<char> {
        self.source[self.current as usize..].chars().next()
    }

    pub fn pop(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.current += c.len_utf8() as i32;
        Some(c)
    }

    pub fn push(&mut self, token_kind: TokenKind) {
        let text = &self.source[self.start as usize..self.current as usize];
        let token = Token(token_kind, text.to_owned(), self.line as usize);
        self.tokens.push(token)
    }

    pub fn push_optional(
        &mut self,
        expected: char,
        expected_token: TokenKind,
        current_token: TokenKind,
    ) {
        let token = if self.match_token(expected) {
            expected_token
        } else {
            current_token
        };
        self.push(token)
    }

    pub fn match_token(&mut self, expected: char) -> bool {
        match self.peek() {
            Some(c) if c == expected => {
                self.pop();
                true
            }
            Some(_) | None => false,
        }
    }

    pub fn tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }
}
