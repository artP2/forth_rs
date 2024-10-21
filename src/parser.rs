use std::process::exit;

use crate::{dictionary::Dictionary, stack::Stack};

pub fn parse(word: &str, stack: &mut Stack, dic: &mut Dictionary) -> () {
    match word {
        "bye" => exit(0),
        "." => println!("{}", stack.pop()),
        "emit" => println!("{}", stack.pop() as u8 as char),
        ".s" => println!("{:?}", stack.see()),
        "+" => stack.sum(),
        "*" => stack.mul(),
        "-" => stack.minus(),
        "/" => stack.div(),
        "mod" => stack.f_mod(),
        "=" => stack.equal(),
        "dup" => stack.dup(),
        "over" => stack.over(),
        "swap" => stack.swap(),
        "drop" => _ = stack.pop(),
        "nip" => _ = stack.nip(),
        "rot" => stack.rot(),
        w if dic.exec(w, stack) => (),
        n if n.parse::<i32>().is_ok() => stack.push(n.parse().unwrap()),
        undefined => println!("{} is not defined", undefined),
    }
}
