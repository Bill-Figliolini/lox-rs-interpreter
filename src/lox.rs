#[derive(Debug, PartialEq)]
enum Tokens {}

fn scan(_source: Vec<u8>) -> Vec<Tokens> {
    Vec::new()
}

pub(super) fn run(source: Vec<u8>) {
    let tokens = scan(source);

    for token in tokens {
        println!("{:?}", token)
    }
}
