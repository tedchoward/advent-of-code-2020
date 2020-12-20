use crate::scanner::{Token, TokenType};

#[derive(Debug)]
pub enum Expr {
    Literal(u64),
    Grouping(Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
}

impl Expr {
    pub fn evaluate(&self) -> u64 {
        match self {
            Expr::Binary(left, operator, right) => {
                let left = left.evaluate();
                let right = right.evaluate();
                match operator.token_type {
                    TokenType::Star => {
                        return left * right;
                    }
                    TokenType::Plus => {
                        return left + right;
                    }
                    _ => panic!("Unexpected TokenType: {:?}", operator.token_type),
                }
            }
            Expr::Grouping(expr) => return expr.evaluate(),
            Expr::Literal(value) => return *value,
        }
    }
}
