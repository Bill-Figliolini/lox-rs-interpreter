use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
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
    Identifier(Vec<u8>),
    String(Vec<u8>),
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

#[derive(Debug, PartialEq, Clone)]
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

fn new_reserved_words() -> HashMap<Vec<u8>, TokenType> {
    let mut reserved_words = HashMap::new();
    //Logical operators and booleans
    reserved_words.insert(b"true".to_vec(), TokenType::True);
    reserved_words.insert(b"false".to_vec(), TokenType::False);
    reserved_words.insert(b"and".to_vec(), TokenType::And);
    reserved_words.insert(b"or".to_vec(), TokenType::Or);
    //Control Flow
    reserved_words.insert(b"if".to_vec(), TokenType::If);
    reserved_words.insert(b"else".to_vec(), TokenType::Else);
    reserved_words.insert(b"for".to_vec(), TokenType::For);
    reserved_words.insert(b"while".to_vec(), TokenType::While);
    //Variable Related
    reserved_words.insert(b"var".to_vec(), TokenType::Var);
    reserved_words.insert(b"this".to_vec(), TokenType::This);
    reserved_words.insert(b"super".to_vec(), TokenType::Super);
    reserved_words.insert(b"nil".to_vec(), TokenType::Nil);
    reserved_words.insert(b"class".to_vec(), TokenType::Class);
    //Functions Related
    reserved_words.insert(b"fun".to_vec(), TokenType::Fun);
    reserved_words.insert(b"return".to_vec(), TokenType::Return);
    reserved_words.insert(b"print".to_vec(), TokenType::Print);
    reserved_words
}
