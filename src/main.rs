mod dictionary;
mod error;
mod parser;
mod stack;

use dictionary::Dictionary;
use error::ErrorKind;
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
            match error {
                ErrorKind::StackUnderFlowError => println!("Error: stack underflow"),
                ErrorKind::UndefinedWordError(w) => println!("Undefined word: {}", w),
                ErrorKind::ExecError => println!("Exec error"),
            }
            continue 'a;
        }
        for word in line {
            if let Err(error) = parse(word, stack, dic) {
                match error {
                    ErrorKind::StackUnderFlowError => println!("Error: stack underflow"),
                    ErrorKind::UndefinedWordError(w) => println!("Undefined word: {}", w),
                    ErrorKind::ExecError => println!("Exec error"),
                }
                continue 'a;
            }
        }
        println!("ok");
    }
}
