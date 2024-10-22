use std::process::exit;

use crate::{dictionary::Dictionary, error::ErrorKind, stack::Stack};

pub fn parse(word: &str, stack: &mut Stack, dic: &mut Dictionary) -> Result<(), ErrorKind> {
    match word {
        "bye" => exit(0),
        "." => {
            println!("{}", stack.pop()?);
            Ok(())
        }
        "emit" => {
            println!("{}", stack.pop()? as u8 as char);
            Ok(())
        }
        ".s" => {
            println!("{:?}", stack.see());
            Ok(())
        }
        "+" => stack.sum(),
        "*" => stack.mul(),
        "-" => stack.minus(),
        "/" => stack.div(),
        "mod" => stack.f_mod(),
        "=" => stack.equal(),
        "dup" => stack.dup(),
        "over" => stack.over(),
        "swap" => stack.swap(),
        "drop" => {
            _ = stack.pop()?;
            Ok(())
        }
        "nip" => {
            _ = stack.nip()?;
            Ok(())
        }
        "rot" => stack.rot(),
        w if dic.exec(w, stack).is_ok() => Ok(()),
        n if n.parse::<i32>().is_ok() => stack.push(n.parse().unwrap()),
        undefined => Err(ErrorKind::UndefinedWordError(undefined.to_string())),
    }
}
