use m68k_reloaded_scanner::{scan, Token};

fn main() {
    println!("Hello, world!");
    let source = "MOVE.W D3, D6";
    let mut errors = Default::default();

    let tokens: Vec<Token> = scan(source, &mut errors).collect();
    for token in &tokens {
        println!("Got token {:?}.", token);
    }
    println!("({} tokens)", tokens.len());
}
