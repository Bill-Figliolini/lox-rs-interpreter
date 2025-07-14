#[derive(Debug, PartialEq)]
pub enum TokenType {
    //Pairwise Single Character
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    //General Single Character
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
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
    Identifier,
    String,
    Number,

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
    literal_id: Option<usize>,
    line: usize,
}
impl Token {
    pub fn new(token_type: TokenType, line: usize) -> Token {
        Token {
            token_type,
            literal_id: None,
            line,
        }
    }
}
