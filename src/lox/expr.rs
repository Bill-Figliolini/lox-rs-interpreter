///Parser Grammar
/// body terms in all caps are terminals
/// expression -> literal | unary | binary | grouping;
/// literal -> NUMBER | STRING | "true" | "false" | "nil";
///  grouping -> "(" expression ")";
///  unary -> ("-" | "!") expression;
///  binary -> expression operator expression
///  operator -> "==" | "!=" | "<=" | ">="
///              | "+" | "-" | "*" | "/";
/// Will require significant divergence from the book, as it makes use of
/// heavy ammounts of inheritance in common Java styles
///
/// Good use case for Rust, though. Usage of enums for the heads and structs for the bodies
/// should do the trick By the same token, consideration should go to other rust-idiomatic
/// approaches

pub enum Expr {
    Literal(Literal),
    Unary(Unary),
    Binary(Binary),
    Grouping(Grouping),
}

pub enum Operator {
    //Member operator
    Dot,
    //Arithmetic Operators
    Minus,
    Plus,
    Slash,
    Star,
    //Logical Operators
    Not,
    And,
    Or,
    //Assignment
    Assignment,
    //Comparisions
    NotEqual,
    Equal,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

pub enum Literal {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}
pub struct Unary {
    operator: Operator,
    right: Box<Expr>,
}
pub struct Binary {
    left: Box<Expr>,
    operator: Operator,
    right: Box<Expr>,
}
pub struct Grouping {
    expression: Box<Expr>,
}
