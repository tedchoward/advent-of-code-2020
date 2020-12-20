use crate::{
    expr::Expr,
    scanner::{Token, TokenType},
};

pub struct AdvancedParser {
    tokens: Vec<Token>,
    current: usize,
}

impl AdvancedParser {
    pub fn new(tokens: &Vec<Token>) -> Self {
        Self {
            tokens: tokens.clone(),
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Expr {
        self.expression()
    }

    fn expression(&mut self) -> Expr {
        let mut expr = self.addition();

        loop {
            match self.peek().token_type {
                TokenType::Star => {
                    self.advance();
                    let operator = self.previous();
                    let right = self.addition();

                    expr = Expr::Binary(Box::new(expr), operator.clone(), Box::new(right));
                }
                _ => break,
            }
        }

        expr
    }

    fn addition(&mut self) -> Expr {
        let mut expr = self.primary();

        loop {
            match self.peek().token_type {
                TokenType::Plus => {
                    self.advance();
                    let operator = self.previous();
                    let right = self.primary();

                    expr = Expr::Binary(Box::new(expr), operator.clone(), Box::new(right));
                }
                _ => break,
            }
        }

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
            }

            TokenType::LeftParen => {
                self.advance();
                let expr = self.expression();
                self.consume(
                    TokenType::RightParen,
                    String::from("Expect ')' after expression."),
                );
                return Expr::Grouping(Box::new(expr));
            }

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
