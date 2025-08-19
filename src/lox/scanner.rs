use crate::lox::common::{Token, TokenType, report_error};
use phf::phf_map;

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

static KEYWORDS: phf::Map<&'static [u8], TokenType> = phf_map! {
    //Logical and boolena
    b"and" => TokenType::And,
    b"or" => TokenType::Or,
    b"true" => TokenType::True,
    b"false" => TokenType::False,
    //Control flow
    b"if" => TokenType::If,
    b"else" => TokenType::Else,
    b"while" => TokenType::While,
    b"for" => TokenType::For,
    //Function and Variables
    b"fun" => TokenType::Fun,
    b"return" => TokenType::Return,
    b"print" => TokenType::Print,
    b"class" => TokenType::Class,
    b"this" => TokenType::This,
    b"super" => TokenType::Super,
    b"var" => TokenType::Var,
    b"nil" => TokenType::Nil,
};

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
    fn scan_tokens(mut self) -> Vec<Token> {
        while let Some(source_byte) = self.source.next() {
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
                digit if digit.is_ascii_digit() => self.scan_number(digit),
                character if is_valid_identifier_start(character) => {
                    self.scan_identifier(character)
                }
                b' ' | b'\t' | b'\r' => {}
                b'\n' => self.current_line += 1,
                _ => {
                    report_error(self.current_line, "Unexpected character.");
                }
            }
        }
        self.push_new_token(TokenType::EOF);
        self.output
    }

    //Separate loop for building the Vec<u8> that will be stored for strings.
    //Does not append unterminated strings to the output stream of tokens, and increments the lines scanned for each newline
    //In the String.
    fn scan_string(&mut self) {
        let mut result_string: Vec<u8> = Vec::new();
        while let Some(character) = self.source.next() {
            match character {
                b'"' => {
                    self.push_new_token(TokenType::String(result_string));
                    return;
                }
                _ => {
                    if character == b'\n' {
                        self.current_line += 1;
                    }
                    result_string.push(character);
                }
            }
        }
        report_error(self.current_line, "Unterminated String");
        return;
    }

    fn scan_number(&mut self, first_digit: u8) {
        let mut unparsed_number: Vec<u8> = Vec::new();
        unparsed_number.push(first_digit);
        while let Some(potential_num) = self.source.peek() {
            eprintln!("Number is: {}", &potential_num.to_string());
            match potential_num {
                d if d.is_ascii_digit() => {
                    unparsed_number.push(
                        self.source
                            .next()
                            .expect("potential_num already verified as not None"),
                    );
                }
                b'_' => _ = self.source.next().expect("unused underscore"),
                b'.' => {
                    self.source
                        .next()
                        .expect("potential_num already verified as not None");
                    if self.source.peek().is_some_and(|d| d.is_ascii_digit()) {
                        unparsed_number.push(b'.');
                        break;
                    }
                    self.push_number_token(unparsed_number);
                    self.push_new_token(TokenType::Dot);
                    return;
                }
                _ => {
                    eprintln!("Number token pushed at _");
                    self.push_number_token(unparsed_number);
                    return;
                }
            }
        }
        while let Some(potential_num) = self.source.peek() {
            eprintln!("Number is: {}", &potential_num.to_string());
            match potential_num {
                d if d.is_ascii_digit() => unparsed_number.push(
                    self.source
                        .next()
                        .expect("potential_num already verified as not None"),
                ),
                b'_' => _ = self.source.next().expect("unused underscore"),
                _ => {
                    break;
                }
            }
        }
        self.push_number_token(unparsed_number);
    }
    fn scan_identifier(&mut self, first_character: u8) {
        let mut identifier: Vec<u8> = Vec::new();
        identifier.push(first_character);
        while let Some(next_char) = self.source.peek() {
            if !is_valid_identifier(*next_char) {
                break;
            }
            identifier.push(
                self.source
                    .next()
                    .expect("next_char already verified as not None"),
            );
        }
        match KEYWORDS.get(identifier.as_ref()) {
            Some(token_type) => {
                self.push_new_token(token_type.clone());
            }
            None => {
                self.push_new_token(TokenType::Identifier(identifier));
            }
        }
    }

    fn push_number_token(&mut self, unparsed_number: Vec<u8>) {
        let parsed_number = String::from_utf8(unparsed_number)
            .expect("Only Ascii characters should be in number")
            .parse::<f64>()
            .expect("Should always be a valid number");
        self.push_new_token(TokenType::Number(parsed_number));
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
fn is_valid_identifier_start(character: u8) -> bool {
    character.is_ascii_alphabetic() || character == b'_'
}
fn is_valid_identifier(character: u8) -> bool {
    character.is_ascii_alphanumeric() || character == b'_'
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
            let input_output_vec = input_vec.into_iter().zip(output_vec);
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
                let input_output_vec = input.into_iter().zip(output);
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
                let input_output_vec = input.into_iter().zip(output);
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
        mod numbers {
            use super::*;
            #[test]
            fn matches_valid_numbers() {
                let input: Vec<u8> = "123.456".bytes().collect();
                let expected_output = assemble_token_array(vec![
                    (TokenType::Number(123.456), 1),
                    (TokenType::EOF, 1),
                ]);
                let actual_output = scan(input);

                assert_eq!(expected_output, actual_output);
            }
            #[test]
            fn does_not_append_dots() {
                let input: Vec<u8> = "123. ".bytes().collect();
                let expected_output = assemble_token_array(vec![
                    (TokenType::Number(123.0), 1),
                    (TokenType::Dot, 1),
                    (TokenType::EOF, 1),
                ]);
                let actual_output = scan(input);

                assert_eq!(expected_output, actual_output);
            }
            #[test]
            fn can_be_spaced_with_underscores_before_dot() {
                let input: Vec<u8> = "123_456.7891011".bytes().collect();
                let expected_output = assemble_token_array(vec![
                    (TokenType::Number(123456.7891011), 1),
                    (TokenType::EOF, 1),
                ]);
                let actual_output = scan(input);

                assert_eq!(expected_output, actual_output);
            }
            #[test]
            fn can_be_spaced_with_underscores_after_dot() {
                let input: Vec<u8> = "123_456.789_1011".bytes().collect();
                let expected_output = assemble_token_array(vec![
                    (TokenType::Number(123456.7891011), 1),
                    (TokenType::EOF, 1),
                ]);
                let actual_output = scan(input);

                assert_eq!(expected_output, actual_output);
            }
        }
        mod identifiers {
            use super::*;
            #[test]
            fn matches_name_starting_with_letter() {
                let input: Vec<u8> = "Hello".bytes().collect();
                let expected_output = assemble_token_array(vec![
                    (TokenType::Identifier(input.clone()), 1),
                    (TokenType::EOF, 1),
                ]);
                let actual_output = scan(input);
                assert_eq!(expected_output, actual_output);
            }
            #[test]
            fn matches_name_starting_with_underscore() {
                let input: Vec<u8> = "_Hello".bytes().collect();
                let expected_output = assemble_token_array(vec![
                    (TokenType::Identifier(input.clone()), 1),
                    (TokenType::EOF, 1),
                ]);
                let actual_output = scan(input);
                assert_eq!(expected_output, actual_output);
            }
            #[test]
            fn matches_name_with_reserved_word_overlap() {
                let input: Vec<u8> = "variant".bytes().collect();
                let expected_output = assemble_token_array(vec![
                    (TokenType::Identifier(input.clone()), 1),
                    (TokenType::EOF, 1),
                ]);
                let actual_output = scan(input);
                assert_eq!(expected_output, actual_output);
            }
            #[test]
            fn allows_numbers_after_identifier_start() {
                let input: Vec<u8> = "Hello123".bytes().collect();
                let expected_output = assemble_token_array(vec![
                    (TokenType::Identifier(input.clone()), 1),
                    (TokenType::EOF, 1),
                ]);
                let actual_output = scan(input);
                assert_eq!(expected_output, actual_output);
            }
            #[test]
            fn does_not_match_name_starting_with_number() {
                let input: Vec<u8> = "1Hello".bytes().collect();
                let expected_output = assemble_token_array(vec![
                    (TokenType::Number(1.0), 1),
                    (TokenType::Identifier("Hello".bytes().collect()), 1),
                    (TokenType::EOF, 1),
                ]);
                let actual_output = scan(input);
                assert_eq!(expected_output, actual_output);
            }
        }
        mod reserved_words {
            use super::*;
            fn reserved_word_test(input: Vec<u8>, result: TokenType) {
                let expected_output: Vec<Token> =
                    vec![Token::new(result, 1), Token::new(TokenType::EOF, 1)];
                let actual_output = scan(input);

                assert_eq!(expected_output, actual_output);
            }
            #[test]
            fn matches_all_reserved_words() {
                for (key, value) in KEYWORDS.entries() {
                    reserved_word_test(key.to_vec(), value.clone());
                }
            }
        }
        mod identifier_and_reserved_performance {
            use super::*;
        }
    }
}
