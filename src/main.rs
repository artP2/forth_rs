mod dictionary;
mod parser;
mod stack;

use dictionary::Dictionary;
use parser::parse;
use stack::Stack;

fn main() {
    let mut lines = std::io::stdin().lines();

    let stack: &mut Stack = &mut Stack::new();
    let dic: &mut Dictionary = &mut Dictionary::new();

    loop {
        let line = lines.next().unwrap().unwrap().to_lowercase();
        let mut line = line.split_whitespace();
        match line.next().unwrap() {
            ":" => dic.compile(&mut line),
            "see" => println!("{:?}", dic.see(line.next().unwrap())),
            "dic_load" => dic.load(line.next().unwrap()),
            "dic_write" => dic.write(line.next().unwrap()),
            word => parse(word, stack, dic),
        }
        for word in line {
            parse(word, stack, dic);
        }
        println!("ok");
    }
}
