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
                    },
                    TokenType::Plus => {
                        return left + right;
                    },
                    _ => panic!("Unexpected TokenType: {:?}", operator.token_type),
                }
            },
            Expr::Grouping(expr) => return expr.evaluate(),
            Expr::Literal(value) => return *value,
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: &Vec<Token>) -> Self {
        Self {
            tokens: tokens.clone(),
            current: 0,
        }
    }

    // fn primary(&mut self) -> Expr {
    //     if match(TokenType::Number())
    // }

    pub fn parse(&mut self) -> Expr {
        self.expression()
    }

    fn expression(&mut self) -> Expr {
        let mut expr = self.primary();

        loop {
            match self.peek().token_type {
                TokenType::Plus | TokenType::Star => {
                    self.advance();
                    let operator = self.previous();
                    let right = self.primary();

                    expr = Expr::Binary(Box::new(expr), operator.clone(), Box::new(right));
                },
                _ => break,
            }
        }

        // while self.matches(&[TokenType::Plus, TokenType::Star]) {
        //     let operator = self.previous();
        //     let right = self.primary();

        //     expr = Expr::Binary(Box::new(expr), operator.clone(), Box::new(right));
        // }

        expr
    }

    fn primary(&mut self) -> Expr {
        if self.is_at_end() {
            panic!("Expect expression")
        }

        match self.peek().token_type {
            TokenType::Number(val) => {
                self.advance();
                return Expr::Literal(val);
            },

            TokenType::LeftParen => {
                self.advance();
                let expr = self.expression();
                self.consume(TokenType::RightParen, String::from("Expect ')' after expression."));
                return Expr::Grouping(Box::new(expr));
            },

            _ => panic!("Expect Expression"),
        }
    }

    fn consume(&mut self, token_type: TokenType, message: String) -> Token {
        if self.check(token_type) {
            return self.advance();
        }

        panic!("Error: {} {:?}", message, self.peek());
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type == token_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        match self.peek().token_type {
            TokenType::EOF => true,
            _ => false,
        }
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
}
