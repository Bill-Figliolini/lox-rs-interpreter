mod lox;
use std::{fs, io, process};

pub fn run_file(input_file: String) {
    let file_contents = fs::read(input_file);
    match file_contents {
        Ok(file_contents) => {
            if let Err(e) = lox::run(file_contents) {
                eprintln!("Error: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Error Reading File: {}", e);
            process::exit(65);
        }
    }
}
pub fn run_sandbox() {
    let stdin = io::stdin();
    let mut line: String = String::new();
    loop {
        print!("> ");
        match stdin.read_line(&mut line) {
            Ok(val) => {
                if val == 0 {
                    break;
                }
                if let Err(e) = lox::run(line.as_bytes().to_vec()) {
                    eprintln!("{}", e);
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
