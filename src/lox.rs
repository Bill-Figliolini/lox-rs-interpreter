use anyhow::{Ok, Result};

mod expr;
mod parser;
mod scanner;
mod token;

pub fn run(source: Vec<u8>) -> Result<()> {
    let tokens = scanner::scan(source);
    for token in tokens {
        println!("{:?}", token)
    }
    Ok(())
}
