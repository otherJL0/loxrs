#![feature(let_chains)]
pub mod lexer;
pub mod repr;
pub mod token;
pub mod trait_extensions;

use crate::lexer::Lexer;
use reedline::{DefaultPrompt, DefaultPromptSegment, FileBackedHistory, Reedline, Signal};
use std::env;
use std::fs;

fn repl() {
    println!("Lox Interpreter");
    let history = Box::new(
        FileBackedHistory::with_file(100, "lox_repl.log".into())
            .expect("Error setting up history with file"),
    );
    let mut line_editor = Reedline::create().with_history(history);
    let prompt = DefaultPrompt::new(
        DefaultPromptSegment::Basic("〈".to_string()),
        DefaultPromptSegment::Empty,
    );

    loop {
        match line_editor.read_line(&prompt) {
            Ok(Signal::Success(buffer)) => {
                let mut lexer = Lexer::new(&buffer);
                match lexer.scan_tokens() {
                    Ok(tokens) => {
                        for token in tokens {
                            println!("{token:?}");
                        }
                    }
                    Err(err) => println!("{err:?}"),
                }
            }
            Ok(Signal::CtrlD | Signal::CtrlC) => {
                println!("\nAborted!");
                break;
            }
            x => {
                println!("Event: {x:?}");
            }
        }
    }
}

fn read_file(lox_file: &str) {
    println!("Reading file: {lox_file}");
    match fs::read_to_string(lox_file) {
        Ok(contents) => println!("{contents}"),
        _ => eprintln!("cannot read file"),
    }
}

pub fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.len() {
        0 => repl(),
        1 => read_file(&args[0]),
        _ => eprintln!("Usage: loxrs PROGRAM.lox"),
    }
}
