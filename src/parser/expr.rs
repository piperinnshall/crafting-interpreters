use std::fmt::Display;

use crate::lexer::token::Token;

pub enum Literal {
    Number(f64),
    Str(String),
    Boolean(bool),
    Nil,
}

pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Literal),
    Unary(Token, Box<Expr>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Expr::Binary(left, op, right) => write!(f, "({} {} {})", op.0, left, right),
            Expr::Grouping(expr) => write!(f, "(Group {})", expr),
            Expr::Literal(literal) => match literal {
                Literal::Number(n) => write!(f, "{}", n),
                Literal::Str(s) => write!(f, "{}", s),
                Literal::Boolean(b) => write!(f, "{}", b),
                Literal::Nil => write!(f, "nil"),
            },
            Expr::Unary(op, right) => write!(f, "({} {})", op.0, right),
        }
    }
}
