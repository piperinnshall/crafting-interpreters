use crate::token::{Token, TokenKind};

type Tk = TokenKind;

#[derive(Default, Clone)]
pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: i32,
    current: i32,
    line: i32,
}

impl Lexer {
    fn from(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            ..Default::default()
        }
    }

    fn push(&mut self, token_kind: TokenKind) {
        let text = &self.source[self.start as usize..self.current as usize];
        let token = Token(token_kind, text.to_owned(), self.line as usize);
        self.tokens.push(token)
    }

    fn peek(&self) -> Option<char> {
        self.source[self.current as usize..].chars().next()
    }

    fn pop(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.current += 1;
        Some(c)
    }
}

fn scan_token(lexer: &mut Lexer) -> Result<(), String> {
    if lexer.peek().is_none() {
        return Ok(())
    }

    let c = lexer.pop().unwrap();

    match c {
        '(' => lexer.push(Tk::LeftParen),
        ')' => lexer.push(Tk::RightParen),
        '{' => lexer.push(Tk::LeftBrace),
        '}' => lexer.push(Tk::RightBrace),
        ',' => lexer.push(Tk::Comma),
        '.' => lexer.push(Tk::Dot),
        '-' => lexer.push(Tk::Minus),
        '+' => lexer.push(Tk::Plus),
        ';' => lexer.push(Tk::Semicolon),
        '*' => lexer.push(Tk::Star),

        _ => return Err(format!("Unexpected Character: '{}'",c))
    };

    Ok(())
}

// fn match_token() -> bool {
// }

pub fn scan_whitespace(source: impl Into<String>) -> Vec<String> {
    source
        .into()
        .split_whitespace()
        .map(|s| s.to_owned())
        .collect()
}
