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

enum Expr {
    Literal(),
    Unary(),
}

pub fn parse(input: Vec<Token>) -> Expr {}
