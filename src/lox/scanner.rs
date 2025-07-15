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
            b'!' => {
                let result = if self.match_next(b'=') {
                    Token::new(TokenType::BangEqual, self.line)
                } else {
                    Token::new(TokenType::Bang, self.line)
                };
                self.output.push(result);
            }
            b'=' => {
                let result = if self.match_next(b'=') {
                    Token::new(TokenType::EqualEqual, self.line)
                } else {
                    Token::new(TokenType::Equal, self.line)
                };
                self.output.push(result);
            }
            b'>' => {
                let result = if self.match_next(b'=') {
                    Token::new(TokenType::GreaterEqual, self.line)
                } else {
                    Token::new(TokenType::Greater, self.line)
                };
                self.output.push(result);
            }
            b'<' => {
                let result = if self.match_next(b'=') {
                    Token::new(TokenType::LessEqual, self.line)
                } else {
                    Token::new(TokenType::Less, self.line)
                };
                self.output.push(result);
            }
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
    fn match_next(&mut self, expected: u8) -> bool {
        match self.source.peek() {
            None => false,
            Some(c) => {
                if *c == expected {
                    self.source.next();
                    true
                } else {
                    false
                }
            }
        }
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
        fn single_char_test(byte: u8, result: TokenType) {
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
                single_char_test(input, output);
            }
        }
    }
    mod double_character_inputs {
        use super::*;
        mod matched {

            use super::*;
            fn double_char_test_positive(byte: u8, result: TokenType) {
                let input = vec![byte, b'='];
                let expected_output: Vec<Token> =
                    vec![Token::new(result, 1), Token::new(TokenType::EOF, 1)];
                let actual_output = scan(input).expect("Scan of known text should not Fail");

                assert_eq!(expected_output, actual_output);
            }
            #[test]
            fn properly() {
                let input = vec![b'!', b'=', b'>', b'<'];
                let output = vec![
                    TokenType::BangEqual,
                    TokenType::EqualEqual,
                    TokenType::GreaterEqual,
                    TokenType::LessEqual,
                ];
                let input_output_vec = input.into_iter().zip(output.into_iter());
                for (input, output) in input_output_vec {
                    double_char_test_positive(input, output);
                }
            }
        }
        mod unmatched {
            use super::*;
            fn double_char_test_negative(byte: u8, result: TokenType) {
                let input = vec![byte, byte];
                let expected_output: Vec<Token> = vec![
                    Token::new(result.clone(), 1),
                    Token::new(result, 1),
                    Token::new(TokenType::EOF, 1),
                ];
                let actual_output = scan(input).expect("Scan of known text should not Fail");

                assert_eq!(expected_output, actual_output);
            }
            #[test]
            fn properly() {
                let input = vec![b'!', b'>', b'<'];
                let output = vec![TokenType::Bang, TokenType::Greater, TokenType::Less];
                let input_output_vec = input.into_iter().zip(output.into_iter());
                for (input, output) in input_output_vec {
                    double_char_test_negative(input, output);
                }
            }
        }
    }
}
