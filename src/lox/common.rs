#[derive(Debug, PartialEq)]
pub enum TokenType {
    //Single Character
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

    //Single or Double Character
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    //Literals
    Identifier(usize),
    String(usize),
    Number(f64),

    //Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    token_type: TokenType,
    line: usize,
}
impl Token {
    pub fn new(token_type: TokenType, line: usize) -> Token {
        Token { token_type, line }
    }
}

pub fn report_error(line_number: usize, error_message: &str) {
    eprintln!("[line {line_number}] Error: {error_message}");
}
