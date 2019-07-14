mod lex;

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::process;
use crate::lex::Tokenizer;


fn main() {

    let file = File::open("example.pos").unwrap_or_else(|err| {
        eprintln!("An error ocurred opening the file: {}", err);
        process::exit(1);
    });

    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| { l.unwrap() }).collect();

    let mut tokenizer = Tokenizer::new(lines);
    let (tokens, errors) = tokenizer.tokenize();

    if !errors.is_empty() {
        println!("\n\nErrors");
        for err in errors {
            println!("{:?}", err);
        }
    } else {
        println!("Tokens");
        for token in tokens {
            println!("{:?}", token);
        }
    }

}