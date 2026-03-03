use crate::token::{Token, TokenKind};

#[derive(Default, Clone)]
pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: i32,
    current: i32,
    line: i32,
    char: i32,
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

    pub fn advance_char(&mut self) {
        self.char += 1
    }

    pub fn advance_line(&mut self) {
        self.line += 1;
        self.char = 0;
    }

    pub fn peek(&self) -> Option<char> {
        self.source[self.current as usize..].chars().next()
    }

    pub fn pop(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.current += c.len_utf8() as i32;
        self.advance_char();
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

    pub fn push_string(&mut self) -> Result<(), String> {
        let char = self.char;
        let line = self.line;
        while let Some(c) = self.peek() {
            match c {
                '"' => {
                    self.pop();
                    let value = &self.source[self.start as usize + 1..self.current as usize - 1];
                    self.push(TokenKind::String(value.to_owned()));
                    return Ok(());
                }
                '\n' => {
                    self.pop();
                    self.advance_line();
                }
                _ => {
                    self.pop();
                }
            }
        }
        Err(format!(
            "Unterminated string: at line '{}', char '{}'",
            line,
            char - 1
        ))
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

    // Getter API

    pub fn tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }

    pub fn line(&self) -> i32 {
        self.line
    }

    pub fn char(&self) -> i32 {
        self.char
    }
}
