use anyhow::{Ok, Result};

mod common;
mod parser;
mod scanner;

pub fn run(source: Vec<u8>) -> Result<()> {
    let tokens = scanner::scan(source);
    for token in tokens {
        println!("{:?}", token)
    }
    Ok(())
}
