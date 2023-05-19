//Disable later
#[allow(dead_code, clippy::module_name_repetitions)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Null,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
    Unknown,
}

// #[derive(Copy, Clone, Debug)] Uncomment this, we don't need to be able to clone this, just copy. Only for DEBUG
#[derive(Clone, Debug)]
pub struct Token {
    kind: TokenType,
    line: u32,
    value: String, // FOR DEBUG ONLY
}

impl Token {
    pub fn new(kind: TokenType, line: u32, value: String) -> Self {
        Self { kind, line, value }
    }
    /*
    pub fn lexeme(&self) -> &Lexeme {
        &self.lexeme
    }
    */
    pub fn kind(&self) -> &TokenType {
        &self.kind
    }
    pub fn line(&self) -> u32 {
        self.line
    }
    pub fn value(&self) -> &String {
        &self.value
    }
}
