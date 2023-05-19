use std::collections::HashMap;

use crate::{
    token::{Token, TokenType},
    utils,
};

pub struct Lexer<'a> {
    src: String,
    start: usize,
    current: usize,
    line: u32,
    tokens: Vec<Token>,
    keywords: HashMap<&'a str, TokenType>,
}

impl<'a> Lexer<'a> {
    pub fn print_tokens(&self) {
        for t in &self.tokens {
            println!(
                "Token: {} | Type: {:?} | Line: {}",
                t.value(),
                t.kind(),
                t.line(),
            );
        }
    }
    pub fn new(src: String) -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("y", TokenType::And);
        keywords.insert("objeto", TokenType::Class);
        keywords.insert("sino", TokenType::Else);
        keywords.insert("falso", TokenType::False);
        keywords.insert("por", TokenType::For);
        keywords.insert("funcion", TokenType::Fun);
        keywords.insert("si", TokenType::If);
        keywords.insert("nada", TokenType::Null);
        keywords.insert("o", TokenType::Or);
        keywords.insert("mostrar", TokenType::Print);
        keywords.insert("devolver", TokenType::Return);
        keywords.insert("super", TokenType::Super);
        keywords.insert("este", TokenType::This);
        keywords.insert("verdadero", TokenType::True);
        keywords.insert("variable", TokenType::Var);
        keywords.insert("mientras", TokenType::While);
        Self {
            src,
            start: 0,
            current: 0,
            line: 1,
            // Increase this number later
            tokens: Vec::with_capacity(20),
            keywords,
        }
    }
    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.add_token(TokenType::Eof);
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.src.len()
    }
    fn scan_token(&mut self) {
        let c = self.advance().unwrap();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.does_next_match('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.does_next_match('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '<' => {
                if self.does_next_match('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            '>' => {
                if self.does_next_match('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }
            '/' => {
                if self.does_next_match('/') {
                    loop {
                        match self.peek() {
                            None => break,
                            Some(c) => {
                                if c == '\n' {
                                    break;
                                }
                            }
                        }
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '"' => self.handle_string(),
            _ => {
                if c.is_numeric() {
                    self.handle_number();
                } else if c.is_alphabetic() {
                    self.handle_identifier();
                } else {
                    self.add_token_with_lexeme(TokenType::Unknown);
                    utils::error(self.line, &format!("Carácter no esperado {c}"));
                }
            }
        }
    }
    fn handle_identifier(&mut self) {
        loop {
            match self.peek() {
                None => break,
                Some(c) => {
                    if !c.is_alphanumeric() {
                        break;
                    }
                }
            }
            self.advance();
        }
        self.add_token_with_lexeme(TokenType::Identifier);
    }
    fn handle_number(&mut self) {
        loop {
            match self.peek() {
                None => break,
                Some(c) => {
                    if c == '.' || c == ',' {
                        match self.peek_next() {
                            Some(c) => {
                                if !c.is_numeric() {
                                    break;
                                }
                            }
                            None => break,
                        }
                    } else if !c.is_numeric() {
                        break;
                    }
                }
            }
            self.advance();
        }
        self.add_token_with_lexeme(TokenType::Number);
    }
    fn handle_string(&mut self) {
        loop {
            match self.peek() {
                None => {
                    utils::error(self.line, "Falta cerrar una string con un \"");
                    break;
                }
                Some(c) => {
                    if c == '\n' {
                        self.line += 1;
                    } else if c == '\"' {
                        break;
                    }
                }
            }
            self.advance();
        }
        self.advance();
        self.add_token_with_lexeme(TokenType::String);
    }
    fn peek(&self) -> Option<char> {
        self.src.chars().nth(self.current)
    }
    fn peek_next(&self) -> Option<char> {
        self.src.chars().nth(self.current + 1)
    }

    fn does_next_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.src.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn add_token(&mut self, kind: TokenType) {
        self.tokens.push(Token::new(
            kind,
            self.line,
            self.src[self.start..self.current].to_string(),
        ));
    }
    fn add_token_with_lexeme(&mut self, kind: TokenType) {
        match kind {
            TokenType::String => self.tokens.push(Token::new(
                kind,
                self.line,
                self.src[self.start + 1..self.current - 1].to_string(),
            )),
            TokenType::Identifier => {
                let token = match self.keywords.get(&self.src[self.start..self.current]) {
                    Some(k) => Token::new(
                        *k,
                        self.line,
                        self.src[self.start..self.current].to_string(),
                    ),
                    None => Token::new(
                        kind,
                        self.line,
                        self.src[self.start..self.current].to_string(),
                    ),
                };
                self.tokens.push(token);
            }
            _ => self.tokens.push(Token::new(
                kind,
                self.line,
                self.src[self.start..self.current].to_string(),
            )),
        }
    }

    fn advance(&mut self) -> Option<char> {
        let res = self.src.chars().nth(self.current);
        self.current += 1;
        res
    }

    // Moves tokens and source out of the lexer
    pub fn transfer_tokens_src(self) -> TokensAndSrc {
        TokensAndSrc {
            tokens: self.tokens,
            src: self.src,
        }
    }
}

pub struct TokensAndSrc {
    pub tokens: Vec<Token>,
    pub src: String,
}
