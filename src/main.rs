// Imports
use std::io::Read;
use std::fs::File;
mod enums;

// Variables
// 8kb probably
const MEM_SIZE: i64 = 8 * 8;

// Lex to tokenize the input
pub fn lex(line: &str) -> Vec<enums::Token> {
    // tokens for iteration
    let tokens = line.split_whitespace();
    // ctokens to return
    let mut ctokens: Vec<enums::Token> = Vec::new();
    // iterate over tokens
    for t in tokens {
        // TODO: lex the tokens and store them in ctokens
        // if it's a comment, break the entire statement
        if t == ";" {
            break;
        }else {
            ctokens.push(enums::Token::Empty);
        }
        
    }
    // return the tokens
    return ctokens;
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
    let code = "MOV AL 61H\nMOV AH 6AH\nINT 21H\n; This is a comment\n";
    let tokens = code.lines();
    for token in tokens {
        run(token);
    }
}