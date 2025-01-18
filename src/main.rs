#![feature(let_chains)]
pub mod lexer;
pub mod token;
pub mod trait_extensions;

use crate::lexer::Lexer;
use reedline::{DefaultPrompt, DefaultPromptSegment, Reedline, Signal};
use std::env;
use std::fs;

fn repl() {
    let mut line_editor = Reedline::create();
    let prompt = DefaultPrompt::new(
        DefaultPromptSegment::Basic("〈".to_string()),
        DefaultPromptSegment::Empty,
    );

    loop {
        match line_editor.read_line(&prompt) {
            Ok(Signal::Success(buffer)) => {
                println!("We processed: {buffer}");
                let mut lexer = Lexer::new(&buffer);
                for token in lexer.scan_tokens() {
                    println!("{token:?}");
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
