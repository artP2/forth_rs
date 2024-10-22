use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    path::Path,
    str::SplitWhitespace,
};

use crate::error::ErrorKind;
use crate::{parser::parse, stack::Stack};

pub struct Dictionary(HashMap<String, Vec<String>>);

impl Dictionary {
    pub fn new() -> Self {
        Dictionary(HashMap::new())
    }

    pub fn exec(&mut self, word: &str, stack: &mut Stack) -> Result<(), ErrorKind> {
        if self.0.contains_key(word) {
            let exec = self.0.get(word).unwrap().to_owned();
            for w in exec {
                if parse(&w, stack, self).is_err() {
                    return Err(ErrorKind::ExecError);
                }
            }
            return Ok(());
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
                self.compile(&mut line)?;
            } else {
                continue;
            }
        }
        Ok(())
    }

    pub fn write(&mut self, file_name: &str) -> Result<(), ErrorKind> {
        let path = Path::new(file_name);
        let mut file = File::create(&path).unwrap();
        let mut content = String::new();

        for word in &self.0 {
            content.push_str(": ");
            content.push_str(word.0);
            for w in word.1 {
                content.push(' ');
                content.push_str(w);
            }
            content.push_str(" ;\n");
        }

        file.write_all(content.as_bytes()).unwrap();
        Ok(())
    }

    pub fn see(&self, word: &str) -> &Vec<String> {
        self.0.get(word).unwrap()
    }

    pub fn compile(&mut self, words: &mut SplitWhitespace) -> Result<(), ErrorKind> {
        let name = words.next().unwrap().to_string();
        let mut tasks = Vec::new();
        for word in words.into_iter() {
            if word == ";" {
                break;
            }
            tasks.push(word.to_string());
        }

        self.0.insert(name, tasks);
        Ok(())
    }
}
