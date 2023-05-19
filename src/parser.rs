use crate::{
    expressions::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr},
    token::{Token, TokenType},
    utils::{self, Error},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Box<Expr> {
        self.expression()
    }

    /* The next functions match expressions based on precedence (lowest first) */

    fn expression(&mut self) -> Box<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Box<Expr> {
        let mut expr = self.comparison();

        // We use a while because we can have more complex equalities
        // Like: 1 == 2 == 3 != 4 == (3 + 2)
        while self.match_type(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison();
            expr = Box::new(Expr::Binary(BinaryExpr::new(expr, right, operator.clone())));
        }
        expr
    }

    fn comparison(&mut self) -> Box<Expr> {
        let mut expr = self.term();

        while self.match_type(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term();
            //expr = Box::new(Expr::Binary(BinaryExpr::new(expr, right, *operator)));
            expr = Box::new(Expr::Binary(BinaryExpr::new(expr, right, operator.clone())));
        }

        expr
    }

    fn term(&mut self) -> Box<Expr> {
        let mut expr = self.factor();

        while self.match_type(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor();
            //expr = Box::new(Expr::Binary(BinaryExpr::new(expr, right, *operator)));
            expr = Box::new(Expr::Binary(BinaryExpr::new(expr, right, operator.clone())));
        }

        expr
    }

    fn factor(&mut self) -> Box<Expr> {
        let mut expr = self.unary();

        while self.match_type(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary();
            //expr = Box::new(Expr::Binary(BinaryExpr::new(expr, right, *operator)));
            expr = Box::new(Expr::Binary(BinaryExpr::new(expr, right, operator.clone())));
        }

        expr
    }

    fn unary(&mut self) -> Box<Expr> {
        if self.match_type(&[TokenType::Bang, TokenType::Minus]) {
            //let right = self.unary();
            let operator = self.previous().clone();
            let right = self.unary();
            //return Box::new(Expr::Unary(UnaryExpr::new(*operator, right)));
            return Box::new(Expr::Unary(UnaryExpr::new(operator.clone(), right)));
        }


        //Fix this, do not unwrap. Handle properly
        self.primary().unwrap()
    }

    //Ugly, refactor this. So far, only primary gets an error and it never gets handled up the chain (unary just unwraps)
    fn primary(&mut self) -> Result<Box<Expr>, Error> {
        if self.match_type(&[
            TokenType::False,
            TokenType::True,
            TokenType::Null,
            TokenType::Number,
            TokenType::String,
        ]) {
            Ok(Box::new(Expr::Literal(LiteralExpr::new(
                self.previous().clone(),
            ))))
        } else if self.match_type(&[TokenType::LeftParen]) {
            let expr = self.expression();
            // Fix this and handle error
            let _ = self.consume(TokenType::RightParen, "Se necesita cerrar el paréntesis");
            Ok(Box::new(Expr::Grouping(GroupingExpr::new(expr))))
        } else {
            Err(self.parse_error(self.peek(), "Se esperaba una expresión"))
        }
    }

    /* ****************************************************** */

    fn consume(&mut self, kind: TokenType, msg: &str) -> Result<&Token, Error> {
        if self.check(kind) {
            Ok(self.advance())
        } else {
            Err(self.parse_error(self.peek(), msg))
        }
    }

    fn parse_error(&self, token: &Token, msg: &str) -> Error {
        utils::error_parse(token, msg);
        Error::ParsingError
    }

    fn match_type(&mut self, kinds: &[TokenType]) -> bool {
        // See if the next token is one of the token types supplied as params
        for kind in kinds {
            if self.check(*kind) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if *self.previous().kind() == TokenType::Semicolon {
                return;
            }

            match self.peek().kind() {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => (),
            }

            self.advance();
        }
    }

    fn is_at_end(&self) -> bool {
        *self.peek().kind() == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens.get(self.current).unwrap()
    }

    fn previous(&self) -> &Token {
        &self.tokens.get(self.current - 1).unwrap()
    }

    fn check(&self, kind: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            *self.peek().kind() == kind
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
}
