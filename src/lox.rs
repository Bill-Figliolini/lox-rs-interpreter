use anyhow::{Ok, Result};

#[derive(Debug, PartialEq)]
struct Token {}

fn scan(_source: Vec<u8>) -> Result<Vec<Token>> {
    Ok(Vec::new())
}

pub(super) fn run(source: Vec<u8>) -> Result<()> {
    let tokens = scan(source)?;
    for token in tokens {
        println!("{:?}", token)
    }
    Ok(())
}
