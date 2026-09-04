#![allow(warnings)]

use std::env;
use std::fs;

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

#[derive(Debug)]
enum Expr {
    Identification(String),       // set a = b
    Integer(i32),                 // set a = 5
}

#[derive(Debug)]
struct SetStmt {
    name: String,                 // variable name
    value: Expr,                  // value if (Integer/Identification)
}

#[derive(Debug)]
struct AST {
    statements: Vec<SetStmt>,
}

pub fn read_file(file: &str) -> String {
    fs::read_to_string(file).unwrap()
}

pub fn build_ast(tokens: Vec<Tokens>) -> AST {
    let mut statements = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {
            Tokens::Declaration(decl)
            if decl == "set" => {
                statements.push(
                    SetStmt {
                        name: "x".to_string(),
                        value: Expr::Integer(5),
                    }
                );
            }
            _ => {
                eprintln!("warning: unknown declaration!");
                return AST { statements };
            }
        }
        i = i + 1;
    }

    AST { statements }
}

pub fn make_tokens(content: &str) -> Vec<Tokens> {
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

fn main() {
    let argv: Vec<String> = env::args().collect();

    if argv.len() < 3 || argv[1] != "-f" {
        eprintln!("usage: rustvn -f <file.vn>");
        return;
    }
    let content = read_file(&argv[2]);
    let tokens = make_tokens(&content);
    let ast = build_ast(tokens);
    
    println!("{:#?}", ast);
}