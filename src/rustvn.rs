#![allow(warnings)]

use std::process::Command;
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
enum Expr {                       // Expresssion
    Identification(String),       // set a = b
    Integer(i32),                 // set a = 5
    Addition(Box<Expr>, Box<Expr>),
    Subtraction(Box<Expr>, Box<Expr>),
    Multiplication(Box<Expr>, Box<Expr>),
    Division(Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
struct SetStmt {                  // Statement
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

pub fn write_file(file: &str, struc: &str) {
    fs::write(file, struc).unwrap();
}

pub fn codegen_expr(expr: &Expr) -> String {
    match expr {
        Expr::Integer(int) => format!("{}", int),
        Expr::Identification(name) => name.clone(),
        Expr::Addition(l, r) => {
            format!("({} + {})", codegen_expr(l), codegen_expr(r))
        }
        Expr::Subtraction(l, r) => {
            format!("({} - {})", codegen_expr(l), codegen_expr(r))
        }
        Expr::Multiplication(l, r) => {
            format!("({} * {})", codegen_expr(l), codegen_expr(r))
        }
        Expr::Division(l, r) => {
            format!("({} / {})", codegen_expr(l), codegen_expr(r))
        }
    }
}

pub fn codegen_exitprogram(asm: &mut String, code: String) {
    let code = code.parse::<u8>().unwrap_or(0);

    if code == 0 {
        asm.push_str("\n    mov eax, 60\n");
        asm.push_str("    xor edi, edi\n");
    } else {
        asm.push_str("\n    mov eax, 60\n");
        asm.push_str(&format!("    mov edi, {}\n", code));
    }
    asm.push_str("    syscall\n");
}

pub fn codegen(aststruct: &AST) -> String {
    let mut asm = String::new();

    asm.push_str("; vn code-generation\n\n");
    asm.push_str("global _start\n");
    asm.push_str("default rel\n\n");
    asm.push_str("section .data\n");
    
    for stmt in &aststruct.statements {
        asm.push_str(&format!("    {}: dq 0\n", stmt.name));
    }

    asm.push_str("\nsection .text\n");
    asm.push_str("_start:\n");

    for stmt in &aststruct.statements {
        let value = codegen_expr(&stmt.value);
        asm.push_str(&format!("    mov rax, {}\n", value));
        asm.push_str(&format!("    mov [{}], rax\n", stmt.name));
    }

    codegen_exitprogram(&mut asm, "0".to_string());

    asm
}

pub fn parse_expr(tokens: &[Tokens], pos: &mut usize) -> Expr {
    let mut left = match &tokens[*pos] {
        Tokens::Integer(int) => Expr::Integer(*int),
        Tokens::Identification(ident) => Expr::Identification(ident.clone()),
        _ => Expr::Integer(0),
    };
    *pos += 1;

    while *pos < tokens.len() {
        match &tokens[*pos] {
            Tokens::Plus => {
                *pos += 1;
                let right = parse_expr(tokens, pos);
                left = Expr::Addition(Box::new(left), Box::new(right));
            }
            Tokens::Minus => {
                *pos += 1;
                let right = parse_expr(tokens, pos);
                left = Expr::Subtraction(Box::new(left), Box::new(right));
            }
            Tokens::Star => {
                *pos += 1;
                let right = parse_expr(tokens, pos);
                left = Expr::Multiplication(Box::new(left), Box::new(right));
            }
            Tokens::Slash => {
                *pos += 1;
                let right = parse_expr(tokens, pos);
                left = Expr::Division(Box::new(left), Box::new(right));
            }
            _ => break,
        }
    }

    left
}

pub fn build_ast(tokens: Vec<Tokens>) -> AST {
    let mut statements = Vec::new();
    let mut tokptr = 0;

    while tokptr < tokens.len() {
        match &tokens[tokptr] {
            Tokens::Declaration(decl) => {
                if decl == "set" {
                    if tokptr + 3 < tokens.len() {
                        let name = match &tokens[tokptr + 1] {
                            Tokens::Identification(name) => name.clone(),
                            _ => {
                                eprintln!("AST: expected identification name");
                                return AST { statements };
                            }
                        };
                        if let Tokens::Equal = &tokens[tokptr + 2] {
                            tokptr += 3;

                            let value = parse_expr(&tokens, &mut tokptr);
                            statements.push(
                            SetStmt {
                                    name,
                                    value
                                }
                            );
                        }
                        tokptr += 4;
                        continue;
                    }
                } else {
                    eprintln!("AST: unknown declaration");
                    return AST { statements };
                }
            }
            _ => {}
        }
        tokptr += 1;
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

    if argv.len() < 4 || argv[1] != "-f" || argv[3] != "-o" {
        eprintln!("usage: rustvn -f <file.vn> -o <file>");
        return;
    }
    println!("rvn: reading content...");
    let content = read_file(&argv[2]);
    println!("rvn: content readen");

    println!("rvn: making tokens...");
    let tokens = make_tokens(&content);

    println!("rvn: tokens maked");
    println!("rvn: building ast...");
    println!("tokens: {:?}", tokens);

    let ast = build_ast(tokens);
    println!("rvn: ast builded");

    println!("rvn: generating code...");
    let asm = codegen(&ast);
    println!("rvn: code generated:");
    println!("\n{}\n", asm);
    
    println!("{:#?}", ast);

    write_file(&argv[4], &asm);
    let asm_file = format!("{}.asm", &argv[4]);
    fs::write(&asm_file, &asm).unwrap();

    let obj_file = format!("{}.o", &argv[4]);
    Command::new("nasm")
        .args(["-f", "elf64", &asm_file, "-o", &obj_file])
        .status()
        .unwrap();
    Command::new("ld")
        .args([&format!("{}.o", &argv[4]), "-o", &argv[4]])
        .status()
        .unwrap();
}