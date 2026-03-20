use crate::lexer::token::{Token, TokenKind};
use crate::parser::expr::{Expr, Literal};

type Tk = TokenKind;

#[derive(Default)]
struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            ..Default::default()
        }
    }

    fn pop(&mut self) -> Option<Token> {
        let t = self.peek()?;
        self.current += 1;
        Some(t)
    }

    fn expr(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        // let mut expr = self.comparison();
        while self.match_token(&[Tk::BangEqual, Tk::EqualEqual]) {
            let operator = self.previous();
            // let right = self.comparison();
            // expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        // expr
        Expr::Literal(Literal::Nil)
    }

    fn match_token(&mut self, expected: &[TokenKind]) -> bool {
        match self.peek() {
            Some(t) if expected.contains(&t.0) => {
                // self.advance();
                true
            }
            Some(_) | None => false,
        }
    }

    fn peek(&self) -> Option<Token> {
        self.tokens.get(self.current).cloned()
    }

    fn previous(&self) -> Option<Token> {
        self.tokens.get(self.current.checked_sub(1)?).cloned()
    }
}
