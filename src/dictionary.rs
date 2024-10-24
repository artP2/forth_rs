use std::fmt::Display;
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write as IoWrite},
    path::Path,
};

use crate::error::ErrorKind;
use crate::parser::{exec_tokens, Token};
use crate::stack::Stack;

pub struct Dictionary(HashMap<String, Vec<Token>>);

impl Dictionary {
    pub fn new() -> Self {
        Dictionary(HashMap::new())
    }

    pub fn exec(&self, word: &str, stack: &mut Stack) -> Result<String, ErrorKind> {
        if self.0.contains_key(word) {
            let tokens = self.0.get(word).unwrap();
            match exec_tokens(&tokens, stack, self) {
                Ok(s) => return Ok(s),
                Err(e) => return Err(e),
            }
        }
        Err(ErrorKind::UndefinedWordError(word.to_string()))
    }

    pub fn load(&mut self, file_name: &str) -> Result<(), ErrorKind> {
        let path = Path::new(file_name);
        let mut file = File::open(&path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();

        let lines = content.lines();

        for line in lines {
            let mut line = line.split_whitespace();
            if line.next().unwrap() == ":" {
                let tokens: Vec<Token> = line.map(|t| t.parse::<Token>().expect("error")).collect();
                self.compile(&tokens)?;
            } else {
                continue;
            }
        }
        Ok(())
    }

    pub fn _write(&self, file_name: &str) -> Result<(), ErrorKind> {
        let path = Path::new(file_name);
        let mut file = File::create(&path).unwrap();

        file.write_all(self.to_string().as_bytes()).unwrap();
        Ok(())
    }

    pub fn see(&self, word: &str) -> String {
        format!("{:?}", self.0.get(word).unwrap())
    }

    pub fn compile(&mut self, tokens: &Vec<Token>) -> Result<(), ErrorKind> {
        let (name, tokens) = tokens.split_first().unwrap();
        let name = name.to_string();
        self.0.insert(name, tokens.to_vec());
        Ok(())
    }
}

impl Display for Dictionary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for word in &self.0 {
            write!(f, ": {}", word.0)?;
            for w in word.1 {
                write!(f, " {}", w)?;
            }
            write!(f, " ;\n")?;
        }
        Ok(())
    }
}
