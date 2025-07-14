mod lox;
use std::{fs, io, process};

pub fn run_file(input_file: String) {
    let file_contents = fs::read(input_file);
    match file_contents {
        Ok(file_contents) => {
            lox::run(file_contents);
        }
        Err(e) => {
            eprintln!("Error Reading File: {}", e.to_string());
            process::exit(65);
        }
    }
}
pub fn run_sandbox() {
    let stdin = io::stdin();
    let mut line: String = String::new();
    loop {
        print!("> ");
        if let Ok(val) = stdin.read_line(&mut line) {
            if val == 0 {
                break;
            }
            lox::run(line.as_bytes().to_vec());
        } else {
            eprintln!("Error: ")
        };
    }
}
