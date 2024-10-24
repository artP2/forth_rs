mod dictionary;
mod error;
mod parser;
mod stack;

use std::env;

use dictionary::Dictionary;
use parser::{handle_mode, Token};
use stack::Stack;

fn main() {
    let stack: &mut Stack = &mut Stack::new();
    let dic: &mut Dictionary = &mut Dictionary::new();

    let args: Vec<String> = env::args().collect();
    if let Some(file_name) = args.get(1) {
        match dic.load(&file_name) {
            Ok(_) => (),
            Err(e) => println!("{e}"),
        }
    }

    let mut lines = std::io::stdin().lines();

    loop {
        let line = lines.next().unwrap().unwrap().to_lowercase();
        let tokens: Vec<Token> = line
            .split_whitespace()
            .map(|t| t.parse::<Token>().expect("error"))
            .collect();
        match handle_mode(&tokens, stack, dic) {
            Ok(s) => println!("{} ok", s),
            Err(e) => println!("{e}"),
        }
    }
}
