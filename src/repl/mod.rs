use crate::lexer::Lexer;
use crate::token::TokenType;
use std::io;
use std::io::{BufRead, Write};

const PROMPT: &'static str = ">> ";

pub fn start(std_in: io::Stdin, mut std_out: io::Stdout) -> io::Result<()> {
    let mut reader = io::BufReader::new(std_in);

    loop {
        std_out.write_all(PROMPT.as_ref())?;
        std_out.flush()?;
        let mut line = String::new();
        reader.read_line(&mut line)?;
        let mut lexer = Lexer::new(&line);
        let mut tok = lexer.next_token();
        while tok.r#type != TokenType::ILLEGAL {
            std_out.write_all(format!("{:?}\n", tok).as_ref())?;
            tok = lexer.next_token();
        }
        std_out.flush()?;
    }
}
