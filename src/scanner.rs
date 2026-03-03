use crate::lexer::Lexer;
use crate::token::{Token, TokenKind};

type Tk = TokenKind;

pub fn scan_tokens(source: impl Into<String>) -> Vec<String> {
    let mut lexer = Lexer::from(source);
    let mut errors = Vec::new();

    while lexer.peek().is_some() {
        lexer.advance_start();

        if let Err(e) = scan_token(&mut lexer) {
            errors.push(e);
        }
    }

    errors.iter().for_each(|e| println!("{}", e));

    lexer.tokens().into_iter().map(|t| t.1).collect()
}

pub fn scan_whitespace(source: impl Into<String>) -> Vec<String> {
    source
        .into()
        .split_whitespace()
        .map(|s| s.to_owned())
        .collect()
}

fn scan_token(lexer: &mut Lexer) -> Result<(), String> {
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
        '!' => lexer.push_optional('=', Tk::BangEqual, Tk::Bang),
        '=' => lexer.push_optional('=', Tk::EqualEqual, Tk::Equal),
        '<' => lexer.push_optional('=', Tk::LessEqual, Tk::Less),
        '>' => lexer.push_optional('=', Tk::GreaterEqual, Tk::Greater),
        '"' => lexer.push_string()?,
        '\n' => lexer.advance_line(),
        '/' => eat_comment(lexer),

        ' ' | '\r' | '\t' => {}

        _ => {
            return Err(format!(
                "Unexpected Character: '{}' at line: '{}', character: '{}'",
                c,
                lexer.line(),
                lexer.char() - 1
            ))
        }
    };

    Ok(())
}

fn eat_comment(lexer: &mut Lexer) {
    if lexer.match_token('/') {
        while matches!(lexer.peek(), Some(c) if c != '\n') {
            lexer.pop();
        }
    } else {
        lexer.push(Tk::Slash);
    }
}
