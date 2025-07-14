use lox_rs_interpreter::{run_file, run_sandbox};
use std::{env, process::exit};

fn main() {
    let mut arguments: Vec<String> = env::args().collect();
    match arguments.len() {
        0 => run_sandbox(),
        1 => run_file(std::mem::take(&mut arguments[1])),
        _ => {
            println!("Usage: lox [file]");
            exit(64)
        }
    }
}
