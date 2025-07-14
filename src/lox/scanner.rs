use std::array::IntoIter;

use crate::lox::common::{Token, TokenType};
use anyhow::Result;

struct Scanner {
    output: Vec<Token>,
    line: usize,
}

impl Scanner {
    fn new() -> Scanner {
        let output = Vec::new();
        let line = 1;
        Scanner { output, line }
    }

    fn scan_tokens(mut self, source_stream: impl IntoIterator) -> Result<Vec<Token>> {
        self.output.push(Token::new(TokenType::EOF, self.line));
        Ok(self.output)
    }
}

pub fn scan(source: Vec<u8>) -> Result<Vec<Token>> {
    let scanner = Scanner::new();
    scanner.scan_tokens(source.into_iter())
}

#[cfg(test)]
mod test {
    use super::*;
    mod empty_input {
        use super::*;
        #[test]
        fn gets_eof() {
            let input: Vec<u8> = Vec::new();
            let expected_output: Vec<Token> = vec![Token::new(TokenType::EOF, 1)];
            let actual_output = scan(input).expect("Scan of known text should not Fail");

            assert_eq!(expected_output, actual_output);
        }
    }
}
