use crate::lexer::token::{Token, TokenKind};
use crate::parser::expr::{Expr, Literal};

type Tk = TokenKind;

#[derive(Default)]
struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn from(tokens: Vec<Token>) -> Self {
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

    fn expr(&mut self) -> Option<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Option<Expr> {
        let mut expr = self.comparison();
        while self.match_token(&[Tk::BangEqual, Tk::EqualEqual]) {
            let operator = self.previous()?;
            let right = self.comparison();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Some(expr)
    }

    fn comparison(&mut self) -> Expr {
        // let mut expr = term();
        while self.match_token(&[Tk::Greater, Tk::GreaterEqual, Tk::Less, Tk::LessEqual]) {
            let operator = self.previous();
            // let right = self.term();
            // expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        // expr
        Expr::Literal(Literal::Nil)
    }

    fn term(&mut self) -> Expr {
        // let mut expr = factor();
        while self.match_token(&[Tk::Minus, Tk::Plus]) {
            let operator = self.previous();
            let right =  self.factor();
            // expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }
        //expr
        Expr::Literal(Literal::Nil)
    }

    fn factor(&mut self) -> Expr {
        // let mut expr = unary();
        while self.match_token(&[Tk::Minus, Tk::Plus]) {
            let operator = self.previous();
            let right =  self.unary();
            // expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }
        //expr
        Expr::Literal(Literal::Nil)
    }

    fn unary(&mut self) -> Option<Expr> {
        while self.match_token(&[Tk::Bang, Tk::Minus]) {
            let operator = self.previous()?;
            let right =  self.unary()?;
            return Some(Expr::Unary(operator, Box::new(right)))
        };
        // primary()
        Some(Expr::Literal(Literal::Nil))
    }

    fn primary(&mut self) -> Option<Expr> {
        if self.match_token(&[Tk::False]) {
            return Expr::Literal(Literal::Boolean(false))
        }
        if self.match_token(&[Tk::True]) {
            return Expr::Literal(Literal::Boolean(true))
        }
        if self.match_token(&[Tk::Nil]) {
            return Expr::Literal(Literal::Boolean(Literal::Nil))
        }
    }

    fn match_token(&mut self, expected: &[TokenKind]) -> bool {
        match self.peek() {
            Some(t) if expected.contains(&t.0) => {
                self.pop();
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
