// Imports
use std::io::Read;
use std::fs::File;
use std::env;
mod enums;

// Variables
// 8kb probably
const MEM_SIZE: i64 = 8 * 8;

// Lex to tokenize the input
pub fn lex(line: &str) -> Vec<enums::Token> {
    let mut operations = Vec::new();

    for token in line.split_whitespace() {
        if token 
    }

    return operations;
}

// Parser generates an AST based on the tokens
pub fn parse(lexed: Vec<enums::Token>) {
    lexed;
}

// Compile function to turn the AST into a program
pub fn compile(ast: &str) {
    let mut mem: Vec<i64> = vec![0, MEM_SIZE];
    ast;
}

// Use all functions above and throw em into one singular function
pub fn run(line: &str) {
    let tokens = lex(line);
    let ast = parse(tokens);

    println!("{:?}", ast);
}

// Main function
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("usage: minisembly <file.mini>");
        std::process::exit(1);
    }

    let filename = &args[1];

    // Read file
    let mut file = File::open(filename).expect("File not found");
    let mut source = String::new();
    file.read_to_string(&mut source).expect("Could not read file");

    let tokens = source.lines();
    for token in tokens {
        run(token);
    }
}