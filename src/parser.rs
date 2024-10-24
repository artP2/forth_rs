use std::{fmt::Display, process::exit, str::FromStr};

use crate::{dictionary::Dictionary, error::ErrorKind, stack::Stack};

#[derive(Debug, Clone)]
pub enum Token {
    CompileMode,
    See,
    Number(i32),
    Bye,
    Dot,
    Emit,
    DotStack,
    Plus,
    Asterisk,
    Minus,
    Slash,
    Equal,
    Mod,
    Dup,
    Over,
    Swap,
    Drop,
    Nip,
    Rot,
    Word(String),
}

impl Token {
    pub fn exec(&self, stack: &mut Stack, dic: &Dictionary) -> Result<String, ErrorKind> {
        match self {
            Token::Number(n) => stack.push(*n),
            Token::Bye => exit(0),
            Token::Dot => stack.dot(),
            Token::Emit => stack.emit(),
            Token::DotStack => stack.dot_stack(),
            Token::Plus => stack.sum(),
            Token::Asterisk => stack.mul(),
            Token::Minus => stack.minus(),
            Token::Slash => stack.div(),
            Token::Equal => stack.equal(),
            Token::Mod => stack.f_mod(),
            Token::Dup => stack.dup(),
            Token::Over => stack.over(),
            Token::Swap => stack.swap(),
            Token::Drop => stack.drop(),
            Token::Nip => stack.nip(),
            Token::Rot => stack.rot(),
            Token::Word(w) => dic.exec(w, stack),
            _ => Err(ErrorKind::UnexpectedToken(self.to_string())),
        }
    }
}

impl FromStr for Token {
    type Err = ErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ":" => Ok(Self::CompileMode),
            "see" => Ok(Self::See),
            "bye" => Ok(Self::Bye),
            "." => Ok(Self::Dot),
            "emit" => Ok(Self::Emit),
            ".s" => Ok(Self::DotStack),
            "+" => Ok(Self::Plus),
            "*" => Ok(Self::Asterisk),
            "-" => Ok(Self::Minus),
            "/" => Ok(Self::Slash),
            "mod" => Ok(Self::Mod),
            "=" => Ok(Self::Equal),
            "dup" => Ok(Self::Dup),
            "over" => Ok(Self::Over),
            "swap" => Ok(Self::Swap),
            "drop" => Ok(Self::Drop),
            "nip" => Ok(Self::Nip),
            "rot" => Ok(Self::Rot),
            n if n.parse::<i32>().is_ok() => Ok(Self::Number(n.parse().unwrap())),
            w => Ok(Self::Word(w.to_lowercase())),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Token::CompileMode => ":",
            Token::See => "see",
            Token::Number(n) => &n.to_string(),
            Token::Bye => "bye",
            Token::Dot => ".",
            Token::Emit => "emit",
            Token::DotStack => ".s",
            Token::Plus => "+",
            Token::Asterisk => "*",
            Token::Minus => "-",
            Token::Slash => "/",
            Token::Equal => "=",
            Token::Mod => "mod",
            Token::Dup => "dup",
            Token::Over => "over",
            Token::Swap => "swap",
            Token::Drop => "drop",
            Token::Nip => "nip",
            Token::Rot => "rot",
            Token::Word(w) => w,
        };

        write!(f, "{}", s)?;
        Ok(())
    }
}

pub fn exec_tokens(
    tokens: &Vec<Token>,
    stack: &mut Stack,
    dic: &Dictionary,
) -> Result<String, ErrorKind> {
    let mut s = String::new();
    let iter = tokens.iter();
    for op in iter {
        s.push_str(&op.exec(stack, dic)?);
    }

    Ok(s)
}

pub fn handle_mode(
    tokens: &Vec<Token>,
    stack: &mut Stack,
    dic: &mut Dictionary,
) -> Result<String, ErrorKind> {
    let mut s = String::new();

    let (first, tokens) = tokens.split_first().unwrap();
    match first {
        Token::CompileMode => dic.compile(&tokens.to_vec())?,
        Token::See => match tokens.first().unwrap() {
            Token::Word(w) => s.push_str(&dic.see(w)),
            t => return Err(ErrorKind::UnexpectedToken(t.to_string())),
        },
        op => {
            s.push_str(&op.exec(stack, dic)?);
            s.push_str(&exec_tokens(&tokens.to_vec(), stack, dic)?);
        }
    };

    Ok(s)
}
