use crate::lox::common::{Token, TokenType, report_error};
use anyhow::Result;

struct Scanner {
    source: std::iter::Peekable<std::vec::IntoIter<u8>>,
    output: Vec<Token>,
    line: usize,
}

impl Scanner {
    fn new(source: Vec<u8>) -> Scanner {
        let source = source.into_iter().peekable();
        let output = Vec::new();
        let line = 1;
        Scanner {
            source,
            output,
            line,
        }
    }

    fn scan_token(&mut self) {
        let source_byte = self
            .source
            .next()
            .expect("Called only when iterator is not empty");
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
            _ => {
                report_error(self.line, "Unexpected character.");
            }
        }
    }

    fn scan_tokens(mut self) -> Result<Vec<Token>> {
        while let Some(_) = self.source.peek() {
            self.scan_token();
        }

        self.output.push(Token::new(TokenType::EOF, self.line));
        Ok(self.output)
    }
}

pub fn scan(source: Vec<u8>) -> Result<Vec<Token>> {
    let scanner = Scanner::new(source);
    scanner.scan_tokens()
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
        fn test_template(byte: u8, result: TokenType) {
            let input = vec![byte];
            let expected_output: Vec<Token> =
                vec![Token::new(result, 1), Token::new(TokenType::EOF, 1)];
            let actual_output = scan(input).expect("Scan of known text should not Fail");

            assert_eq!(expected_output, actual_output);
        }
        use super::*;
        #[test]
        fn matches_properly() {
            let input_vec = vec![b'(', b')', b'{', b'}', b',', b'.', b'-', b'+', b';', b'*'];
            let output_vec = vec![
                TokenType::LeftParen,
                TokenType::RightParen,
                TokenType::LeftBrace,
                TokenType::RightBrace,
                TokenType::Comma,
                TokenType::Dot,
                TokenType::Minus,
                TokenType::Plus,
                TokenType::Semicolon,
                TokenType::Star,
            ];
            let input_output_vec = input_vec.into_iter().zip(output_vec.into_iter());
            for (input, output) in input_output_vec {
                test_template(input, output);
            }
        }
    }
}
