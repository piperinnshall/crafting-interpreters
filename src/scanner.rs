use crate::token::TokenKind;
use crate::lexer::Lexer;

type Tk = TokenKind;

fn scan_token(lexer: &mut Lexer) -> Result<(), String> {
    if lexer.peek().is_none() {
        return Ok(());
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
        '!' => lexer.push_optional('=', Tk::BangEqual, Tk::Bang),
        '=' => lexer.push_optional('=', Tk::EqualEqual, Tk::Equal),
        '<' => lexer.push_optional('=', Tk::LessEqual, Tk::Less),
        '>' => lexer.push_optional('=', Tk::GreaterEqual, Tk::Greater),
        '/' => eat_comment(lexer),

        _ => return Err(format!("Unexpected Character: '{}'", c)),
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

pub fn scan_whitespace(source: impl Into<String>) -> Vec<String> {
    source
        .into()
        .split_whitespace()
        .map(|s| s.to_owned())
        .collect()
}
