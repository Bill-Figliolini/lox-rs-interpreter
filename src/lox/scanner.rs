use crate::lox::common::{Token, TokenType, report_error};

// Contains state for the process of scanning through code that the user input
//
// ## Member Variables:
// source - Holds a pointer to the current index in the code that is being Scanned.
//           Can peek ahead one space
// output - Vector of Tokens that will be emitted once scanning is complete
// line   - The current Line Number,
struct Scanner {
    source: std::iter::Peekable<std::vec::IntoIter<u8>>,
    output: Vec<Token>,
    current_line: usize,
}

impl Scanner {
    // ## Input:
    //     Consumes a vector of bytes, which will be converted into a peekable iterator
    //     for rapid traversal and ease of storage
    // ## Output:
    //     struct Scanner, with default values set with an empty Vec<Token> and line 1
    fn new(source: Vec<u8>) -> Scanner {
        let source = source.into_iter().peekable();
        let output = Vec::new();
        let current_line = 1;
        Scanner {
            source,
            output,
            current_line,
        }
    }

    fn push_new_token(&mut self, token_type: TokenType) {
        self.output.push(Token::new(token_type, self.current_line));
    }
    //Primary function for handling scanning the input bytes.
    //  Consumes each byte in the source iterator, matches it with the corresponding token type, and makes a token
    //  Works with 1 byte of lookahead, as in the case of slashes for comments, and double-character operators like ==
    fn scan_token(&mut self) {
        let source_byte = self
            .source
            .next()
            .expect("Called only when iterator is not empty");
        match source_byte {
            b'(' => self.push_new_token(TokenType::LeftParen),
            b')' => self.push_new_token(TokenType::RightParen),

            b'{' => self.push_new_token(TokenType::LeftBrace),
            b'}' => self.push_new_token(TokenType::RightBrace),

            b',' => self.push_new_token(TokenType::Comma),
            b'.' => self.push_new_token(TokenType::Dot),

            b'-' => self.push_new_token(TokenType::Minus),
            b'+' => self.push_new_token(TokenType::Plus),
            b'*' => self.push_new_token(TokenType::Star),

            b';' => self.push_new_token(TokenType::Semicolon),
            b'!' => {
                let result = if self.match_next(b'=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.push_new_token(result);
            }
            b'=' => {
                let result = if self.match_next(b'=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.push_new_token(result);
            }
            b'>' => {
                let result = if self.match_next(b'=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.push_new_token(result);
            }
            b'<' => {
                let result = if self.match_next(b'=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.push_new_token(result);
            }
            b'\\' => {
                if self.match_next(b'\\') {
                    loop {
                        match self.source.peek() {
                            None | Some(b'\n') => break,
                            Some(_) => {
                                self.source.next();
                            }
                        }
                    }
                } else {
                    self.push_new_token(TokenType::Slash)
                }
            }
            b'"' => self.scan_string(),
            b' ' | b'\t' | b'\r' => {}
            b'\n' => self.current_line += 1,
            _ => {
                report_error(self.current_line, "Unexpected character.");
            }
        }
    }

    //Separate loop for building the Vec<u8> that will be stored for strings.
    //Does not append unterminated strings to the output stream of tokens, and increments the lines scanned for each newline
    //In the String.
    fn scan_string(&mut self) {
        let mut result_string: Vec<u8> = Vec::new();
        loop {
            match self.source.next() {
                Some(b'"') => {
                    self.push_new_token(TokenType::String(result_string));
                    return;
                }
                Some(byte) => {
                    if byte == b'\n' {
                        self.current_line += 1;
                    }
                    result_string.push(byte);
                }
                None => {
                    report_error(self.current_line, "Unterminated String");
                    return;
                }
            }
        }
    }
    fn scan_tokens(mut self) -> Vec<Token> {
        while self.source.peek().is_some() {
            self.scan_token();
        }

        self.push_new_token(TokenType::EOF);
        self.output
    }

    //Peeks ahead for potential double-character matches.
    //Returns true and pops if the peek is what the calling function expects, and false otherswise
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
// Scan takes in a Vec of valid UTF8 bytes, and converts them into a Vec of Tokens for later processing.
pub fn scan(source: Vec<u8>) -> Vec<Token> {
    let scanner = Scanner::new(source);
    scanner.scan_tokens()
}

#[cfg(test)]
mod test {
    use super::*;
    fn assemble_token_array(input: Vec<(TokenType, usize)>) -> Vec<Token> {
        input
            .into_iter()
            .map(|(tt, ln)| Token::new(tt, ln))
            .collect()
    }

    mod empty_input {
        use super::*;
        #[test]
        fn gets_eof() {
            let input: Vec<u8> = Vec::new();
            let expected_output: Vec<Token> = vec![Token::new(TokenType::EOF, 1)];
            let actual_output = scan(input);

            assert_eq!(expected_output, actual_output);
        }
    }
    mod single_character_inputs {
        fn single_char_test(byte: u8, result: TokenType) {
            let input = vec![byte];
            let expected_output: Vec<Token> =
                vec![Token::new(result, 1), Token::new(TokenType::EOF, 1)];
            let actual_output = scan(input);

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
                let actual_output = scan(input);

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
                let actual_output = scan(input);

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
        mod comments {
            use super::*;
            #[test]
            fn double_slash_ignores_rest_until_end_of_line() {
                let input: Vec<u8> = "()\\\\qwerty {}\n {}".bytes().collect();
                let expected_output = assemble_token_array(vec![
                    (TokenType::LeftParen, 1),
                    (TokenType::RightParen, 1),
                    (TokenType::LeftBrace, 2),
                    (TokenType::RightBrace, 2),
                    (TokenType::EOF, 2),
                ]);

                let actual_output = scan(input);
                assert_eq!(expected_output, actual_output);
            }
            #[test]
            fn single_slashes_are_not_comments_and_do_not_ignore() {
                let input: Vec<u8> = "()\\ \\{}\n {}".bytes().collect();
                let expected_output = assemble_token_array(vec![
                    (TokenType::LeftParen, 1),
                    (TokenType::RightParen, 1),
                    (TokenType::Slash, 1),
                    (TokenType::Slash, 1),
                    (TokenType::LeftBrace, 1),
                    (TokenType::RightBrace, 1),
                    (TokenType::LeftBrace, 2),
                    (TokenType::RightBrace, 2),
                    (TokenType::EOF, 2),
                ]);

                let actual_output = scan(input);
                assert_eq!(expected_output, actual_output);
            }
        }
    }
    mod literals {
        use super::*;
        mod string {
            use super::*;
            #[test]
            fn matches_enclosed_in_string() {
                let input: Vec<u8> = "\"Hello, World! ()\"".bytes().collect();
                let expected_output = assemble_token_array(vec![
                    (TokenType::String("Hello, World! ()".bytes().collect()), 1),
                    (TokenType::EOF, 1),
                ]);
                let actual_output = scan(input);

                assert_eq!(expected_output, actual_output);
            }
            #[test]
            fn updates_line_count_on_newline_in_string() {
                let input: Vec<u8> = "\"Hello, World!\n ()\"".bytes().collect();
                let expected_output = assemble_token_array(vec![
                    (TokenType::String("Hello, World!\n ()".bytes().collect()), 2),
                    (TokenType::EOF, 2),
                ]);
                let actual_output = scan(input);

                assert_eq!(expected_output, actual_output);
            }

            #[test]
            fn errors_on_unenclosed_string() {
                let input: Vec<u8> = "\"Hello, World! ()".bytes().collect();
                let expected_output = assemble_token_array(vec![(TokenType::EOF, 1)]);
                let actual_output = scan(input);

                assert_eq!(expected_output, actual_output);
            }
        }
        mod nubmers {
            use super::*;
        }
        mod identifiers {
            use super::*;
        }
    }
}
