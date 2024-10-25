mod dictionary;
mod error;
mod parser;
mod stack;

use std::env;

use dictionary::Dictionary;
use nu_ansi_term::{Color, Style};
use parser::{handle_mode, Token};
use reedline::{
    DefaultHinter, DefaultPrompt, DefaultPromptSegment, ExampleHighlighter, FileBackedHistory,
    Reedline, Signal,
};
use stack::Stack;

fn main() {
    let stack: &mut Stack = &mut Stack::new();
    let dic: &mut Dictionary = &mut Dictionary::new();

    let mut commands = parser::commands();

    let args: Vec<String> = env::args().collect();
    if let Some(file_name) = args.get(1) {
        match dic.load(&file_name, &mut commands) {
            Ok(_) => (),
            Err(e) => println!("{e}"),
        }
    }

    let history = Box::new(
        FileBackedHistory::with_file(5, "history.txt".into())
            .expect("Error configuring history with file"),
    );
    let mut line_editor = Reedline::create()
        .with_history(history)
        .with_hinter(Box::new(
            DefaultHinter::default().with_style(Style::new().italic().fg(Color::DarkGray)),
        ))
        .with_highlighter(Box::new(ExampleHighlighter::new(commands)));
    let prompt = DefaultPrompt {
        left_prompt: DefaultPromptSegment::Basic("> ".to_string()),
        right_prompt: DefaultPromptSegment::CurrentDateTime,
    };

    loop {
        let sig = line_editor.read_line(&prompt);
        match sig {
            Ok(Signal::Success(line)) => {
                let tokens: Vec<Token> = line
                    .split_whitespace()
                    .map(|t| t.parse::<Token>().expect("error"))
                    .collect();
                match handle_mode(&tokens, stack, dic) {
                    Ok(s) => println!("{} ok", s),
                    Err(e) => println!("{e}"),
                }
            }
            Ok(Signal::CtrlC) | Ok(Signal::CtrlD) => {
                println!("Aborted");
                break;
            }
            Err(e) => println!("Error: {}", e),
        };
    }
}
