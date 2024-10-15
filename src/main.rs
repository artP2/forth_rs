use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    path::Path,
    process::exit,
    str::SplitWhitespace,
};

fn main() {
    let mut lines = std::io::stdin().lines();

    let mut stack: Vec<i32> = Vec::new();
    let mut dic: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let line = lines.next().unwrap().unwrap().to_lowercase();
        if interpret_line(&line, &mut dic, &mut stack) {
            println!("ok");
        } else {
            println!("not ok");
        }
    }
}

fn interpret_line(
    line: &String,
    dic: &mut HashMap<String, Vec<String>>,
    stack: &mut Vec<i32>,
) -> bool {
    let mut line = line.split_whitespace();
    match line.next().unwrap() {
        ":" => compile_mode(&mut line, dic),
        "see" => println!("{:?}", dic.get(line.next().unwrap())),
        "dic_load" => dic_load(line.next().unwrap(), dic, stack),
        "dic_write" => dic_write(line.next().unwrap(), dic),
        word => parse(word, stack, dic),
    }
    for word in line {
        parse(word, stack, dic);
    }

    true
}

fn compile_mode(words: &mut SplitWhitespace, dic: &mut HashMap<String, Vec<String>>) -> () {
    let name = words.next().unwrap().to_string();
    let mut tasks = Vec::new();
    for word in words.into_iter() {
        if word == ";" {
            break;
        }
        tasks.push(word.to_string());
    }

    dic.insert(name, tasks);
}

fn parse(word: &str, stack: &mut Vec<i32>, dic: &mut HashMap<String, Vec<String>>) -> () {
    let stack_len = stack.len();
    match word {
        "exit" => exit(0),
        "+" => sum(stack),
        "*" => mul(stack),
        "-" => minus(stack),
        "/" => div(stack),
        "mod" => f_mod(stack),
        "=" => equal(stack),
        "." => println!("{}", stack.pop().unwrap()),
        "dup" => stack.push(stack.last().unwrap().clone()),
        "emit" => println!("{}", stack.pop().unwrap() as u8 as char),
        "over" => stack.push(stack.get(stack_len - 2).unwrap().clone()),
        "swap" => stack.swap(stack_len - 1, stack_len - 2),
        "drop" => _ = stack.pop().unwrap(),
        w if dic.contains_key(w) => dic_exec(w, stack, dic),
        n if n.parse::<i32>().is_ok() => stack.push(n.parse().unwrap()),
        undefined => println!("{} is not defined", undefined),
    }
}

fn sum(stack: &mut Vec<i32>) -> () {
    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();
    stack.push(b + a);
}

fn minus(stack: &mut Vec<i32>) -> () {
    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();
    stack.push(b - a);
}

fn div(stack: &mut Vec<i32>) -> () {
    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();
    stack.push(b / a);
}

fn mul(stack: &mut Vec<i32>) -> () {
    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();
    stack.push(b * a);
}

fn f_mod(stack: &mut Vec<i32>) -> () {
    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();
    stack.push(b % a);
}

fn equal(stack: &mut Vec<i32>) -> () {
    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();
    if a == b {
        stack.push(-1); // true
    } else {
        stack.push(0); // false
    }
}

fn dic_exec(word: &str, stack: &mut Vec<i32>, dic: &mut HashMap<String, Vec<String>>) -> () {
    let exec = dic.get(word).unwrap().to_owned();
    for w in exec {
        parse(&w, stack, dic);
    }
}

fn dic_load(file_name: &str, dic: &mut HashMap<String, Vec<String>>, stack: &mut Vec<i32>) -> () {
    let path = Path::new(file_name);
    let mut file = File::open(&path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let lines = content.lines();

    for line in lines {
        _ = interpret_line(&line.to_string(), dic, stack);
    }
}

fn dic_write(file_name: &str, dic: &mut HashMap<String, Vec<String>>) -> () {
    let path = Path::new(file_name);
    let mut file = File::create(&path).unwrap();
    let mut content = String::new();

    for word in dic {
        content.push_str(": ");
        content.push_str(word.0);
        for w in word.1 {
            content.push(' ');
            content.push_str(w);
        }
        content.push_str(" ;\n");
    }

    file.write_all(content.as_bytes()).unwrap();
}
