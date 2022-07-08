#![allow(dead_code)]
#![allow(unused_assignments)]

use std::io;

mod lexer;
mod repl;
mod token;
mod ast;
mod parser;

fn main() {
    println!(
        "Hello {}! This is the Monkey programming language!",
        whoami::username()
    );
    println!("Feel free to type in commands");
    repl::start(io::stdin(), io::stdout()).expect("Failed to execute repl");
}
