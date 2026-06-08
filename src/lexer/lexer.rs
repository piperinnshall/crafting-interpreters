use crate::lexer::token::{Token, TokenKind};

#[derive(Default, Clone)]
pub struct Lexer {
    source: String,
    start: usize,
    current: usize,
    line: i32,
    char: i32,
    start_line: i32,
    start_char: i32,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn from(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            ..Default::default()
        }
    }

    pub fn advance_start(&mut self) {
        self.start = self.current;
        self.start_line = self.line;
        self.start_char = self.char;
    }

    pub fn advance_line(&mut self) {
        self.line += 1;
        self.char = 0;
    }

    pub fn pop(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.current += c.len_utf8();
        self.char += 1;
        Some(c)
    }

    pub fn pop_while(&mut self, predicate: impl Fn(char) -> bool) {
        while matches!(self.peek(), Some(c) if predicate(c)) {
            self.pop();
        }
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

    pub fn push(&mut self, token_kind: TokenKind) {
        let token = Token(
            token_kind,
            self.start_line as usize,
            self.start_char as usize,
        );
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

    pub fn peek(&self) -> Option<char> {
        self.source[self.current..].chars().next()
    }

    pub fn peek_next(&self) -> Option<char> {
        self.source[self.current..].chars().nth(1)
    }

    pub fn lexeme(&self) -> &str {
        &self.source[self.start..self.current]
    }

    pub fn start_line(&self) -> i32 {
        self.start_line
    }

    pub fn start_char(&self) -> i32 {
        self.start_char
    }

    pub fn tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }
}
