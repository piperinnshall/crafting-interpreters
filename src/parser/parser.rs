use crate::lexer::token::{Token, TokenKind};
use crate::parser::expr::{Expr, Literal};

type Tk = TokenKind;

#[derive(Default)]
struct Parser {
    tokens: Vec<Token>,
    current: i32,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            ..Default::default()
        }
    }

    fn expr(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        while self.match_token(&[Tk::BangEqual, Tk::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        expr
    }

    fn match_token(&mut self, expected: &[TokenKind]) -> bool {
        match self.peek() {
            Some(t) if expected.contains(&t.0) => {
                self.advance();
                true
            }
            Some(_) | None => false,
        }
    }

}
