use crate::lexer::token::Token;

enum Expr {
    Binary{ left: Box<Expr> , op: Token, right: Box<Expr> },
}
