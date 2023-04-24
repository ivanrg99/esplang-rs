use crate::token::{Token, TokenType};

#[derive(Debug)]
pub enum Error {
    ParsingError,
}

fn report(line: u32, what: &str, message: &str) {
    println!("[Línea {}] Error {}: {}", line, what, message);
}

pub fn error(line: u32, message: &str) {
    report(line, "", message);
}

pub fn error_parse(token: &Token, message: &str) {
    if *token.kind() == TokenType::Eof {
        report(token.line(), " al final del código", message);
    } else {
        report(token.line(), &format!(" en '{}'", token.value()), message);
    }
}
