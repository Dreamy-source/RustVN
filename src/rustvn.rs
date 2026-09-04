#![allow(warnings)]

use std::env;
use std::fs;

pub fn read_file(file: &str) -> String {
    fs::read_to_string(file).unwrap()
}

pub fn to_tokens(content: &str) -> Vec<Tokens> {
    content.split_whitespace().map(|part|
        match part {
            "set" => Tokens::Declaration("set".to_string()),
            "+" => Tokens::Plus,
            "-" => Tokens::Minus,
            "*" => Tokens::Star,
            "/" => Tokens::Slash,
            "=" => Tokens::Equal,
            _ => {
                if let Ok(int) = part.parse::<i32>() {
                    Tokens::Integer(int)
                } else {
                    Tokens::Identification(part.to_string())
                }
            }
        }).collect()
}

#[derive(Debug)]
enum Tokens {
    Declaration(String),
    Identification(String),
    Integer(i32),
    Equal,
    Plus,
    Minus,
    Star,
    Slash,
}

fn main() {
    let argv: Vec<String> = env::args().collect();

    if argv.len() < 3 || argv[1] != "-f" {
        eprintln!("usage: rustvn -f <file.vn>");
        return;
    }
    let content = read_file(&argv[2]);
    let tokens = to_tokens(&content);

    println!("{:?}", tokens);
}