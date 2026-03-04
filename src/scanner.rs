use crate::lexer::Lexer;
use crate::token::{Token, TokenKind};

type Tk = TokenKind;

pub fn scan_tokens(source: impl Into<String>) -> Vec<Token> {
    let mut lexer = Lexer::from(source);
    let mut errors = Vec::new();
    while lexer.peek().is_some() {
        lexer.advance_start();
        if let Err(e) = scan_token(&mut lexer) {
            errors.push(e);
        }
    }
    errors.iter().for_each(|e| println!("{}", e));
    lexer.tokens()
}

fn scan_token(lexer: &mut Lexer) -> Result<(), String> {
    let c = lexer.pop().unwrap();
    match c {
        ' ' | '\r' | '\t' => {}
        '\n' => lexer.advance_line(),
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
        '"' => scan_string(lexer)?,
        _ if c.is_ascii_digit() => scan_number(lexer),
        _ if c.is_ascii_alphanumeric() => scan_identifier(lexer),
        _ => {
            return Err(format!(
                "Unexpected Character: '{}' at line: '{}', character: '{}'",
                c,
                lexer.start_line(),
                lexer.start_char()
            ))
        }
    };
    Ok(())
}

fn eat_comment(lexer: &mut Lexer) {
    if lexer.match_token('/') {
        lexer.pop_while(|c| c != '\n');
    } else {
        lexer.push(Tk::Slash);
    }
}

fn scan_string(lexer: &mut Lexer) -> Result<(), String> {
    while let Some(c) = lexer.peek() {
        match c {
            '"' => {
                lexer.pop();
                let lexeme = lexer.lexeme();
                let lexeme = &lexeme[1..lexeme.len() - 1];
                lexer.push(TokenKind::String(lexeme.to_owned()));
                return Ok(());
            }
            '\n' => {
                lexer.pop();
                lexer.advance_line();
            }
            _ => { lexer.pop(); }
        }
    }
    Err(format!(
        "Unterminated string: at line '{}', char '{}'",
        lexer.start_line(),
        lexer.start_char()
    ))
}

fn scan_number(lexer: &mut Lexer) {
    lexer.pop_while(|c| c.is_ascii_digit());
    if lexer.peek() == Some('.') && lexer.peek_next().map_or(false, |c| c.is_ascii_digit()) {
        lexer.pop(); // consume the "."
        lexer.pop_while(|c| c.is_ascii_digit());
    }
    lexer.push(Tk::Number(lexer.lexeme().parse().unwrap()));
}


fn scan_identifier(lexer: &mut Lexer) {
    lexer.pop_while(|c| c.is_ascii_alphanumeric() || c == '_');
    let token = match lexer.lexeme() {
        "and" => Tk::And,
        "class" => Tk::Class,
        "else" => Tk::Else,
        "false" => Tk::False,
        "fun" => Tk::Fun,
        "for" => Tk::For,
        "if" => Tk::If,
        "nil" => Tk::Nil,
        "or" => Tk::Or,
        "print" => Tk::Print,
        "return" => Tk::Return,
        "super" => Tk::Super,
        "this" => Tk::This,
        "true" => Tk::True,
        "var" => Tk::Var,
        "while" => Tk::While,
        _ => Tk::Identifier(lexer.lexeme().to_owned()),
    };
    lexer.push(token);
}
