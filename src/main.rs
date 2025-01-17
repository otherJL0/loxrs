#![feature(let_chains)]
pub mod parser;
pub mod repr;
pub mod scanner;
pub mod token;
pub mod trait_extensions;

use crate::parser::Parser;
use crate::repr::AstPrinter;
use crate::scanner::Scanner;
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
        let sig = line_editor.read_line(&prompt);
        match sig {
            Ok(Signal::Success(buffer)) => {
                println!("We processed: {buffer}");
                let mut scanner = Scanner::new(buffer);
                scanner.scan_tokens();
                let mut parser = Parser::new(scanner.tokens);
                let expr = parser.parse();
                let ast_printer = AstPrinter::default();
                let ast_string = ast_printer.print(&expr);
                println!("{ast_string}");
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
