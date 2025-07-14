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

    fn scan_token(&mut self, source_byte: u8) {
        match source_byte {
            b'(' => self
                .output
                .push(Token::new(TokenType::LeftParen, self.line)),
            b')' => self
                .output
                .push(Token::new(TokenType::RightParen, self.line)),

            b'{' => self
                .output
                .push(Token::new(TokenType::LeftBrace, self.line)),
            b'}' => self
                .output
                .push(Token::new(TokenType::RightBrace, self.line)),

            b',' => self.output.push(Token::new(TokenType::Comma, self.line)),
            b'.' => self.output.push(Token::new(TokenType::Dot, self.line)),

            b'-' => self.output.push(Token::new(TokenType::Minus, self.line)),
            b'+' => self.output.push(Token::new(TokenType::Plus, self.line)),
            b'*' => self.output.push(Token::new(TokenType::Star, self.line)),

            b';' => self
                .output
                .push(Token::new(TokenType::Semicolon, self.line)),
            _ => {}
        }
    }

    fn scan_tokens(mut self, source: Vec<u8>) -> Result<Vec<Token>> {
        let mut source_stream = source.into_iter();
        while let Some(c) = source_stream.next() {
            self.scan_token(c);
        }

        self.output.push(Token::new(TokenType::EOF, self.line));
        Ok(self.output)
    }
}

pub fn scan(source: Vec<u8>) -> Result<Vec<Token>> {
    let scanner = Scanner::new();
    scanner.scan_tokens(source)
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
    mod single_character_inputs {
        use super::*;
        #[test]
        fn matches_properly() {}
    }
}
