mod dictionary;
mod error;
mod parser;
mod stack;

use dictionary::Dictionary;
use parser::parse;
use stack::Stack;

fn main() {
    let mut lines = std::io::stdin().lines();

    let stack: &mut Stack = &mut Stack::new();
    let dic: &mut Dictionary = &mut Dictionary::new();

    'a: loop {
        let line = lines.next().unwrap().unwrap().to_lowercase();
        let mut line = line.split_whitespace();
        let ok = match line.next().unwrap() {
            ":" => dic.compile(&mut line),
            "see" => {
                println!("{:?}", dic.see(line.next().unwrap()));
                Ok(())
            }
            "dic_load" => dic.load(line.next().unwrap()),
            "dic_write" => dic.write(line.next().unwrap()),
            word => parse(word, stack, dic),
        };
        if let Err(error) = ok {
            println!("{error}");
            continue 'a;
        }
        for word in line {
            if let Err(error) = parse(word, stack, dic) {
                println!("{error}");
                continue 'a;
            }
        }
        println!("ok");
    }
}
