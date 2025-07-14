use anyhow::{Ok, Result};

mod common;
mod scanner;
use common::*;

pub fn run(source: Vec<u8>) -> Result<()> {
    let tokens = scanner::scan(source)?;
    for token in tokens {
        println!("{:?}", token)
    }
    Ok(())
}
