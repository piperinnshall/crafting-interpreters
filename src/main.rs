mod scanner;
mod lexer;
mod token;

use std::{
    env,
    fs,
    io::{self, Write},
    process,
};


fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        len if len > 2 => {
            eprintln!("Usage: jx [script]");
            process::exit(64);
        }
        2 => {
            if let Err(e) = run_file(&args[1]) {
                eprintln!("Application error: {}", e);
                process::exit(1);
            }
        }
        _ => {
            if let Err(e) = run_prompt() {
                eprintln!("Prompt error: {}", e);
                process::exit(1);
            }
        }
    }
}

fn run_file(path: &str) -> io::Result<()> {
    let input = fs::read_to_string(path)?;
    let mut tokens = scanner::scan_tokens(&input);
    tokens.push(token::Token(token::TokenKind::Eof, 0, 0));
    debug_tokens(&tokens);
    Ok(())
}

fn run_prompt() -> io::Result<()> {
    loop {
        print!("> ");
        io::stdout().flush()?;
        let mut stdin = String::new();
        let bytes_read = io::stdin().read_line(&mut stdin)?;
        if bytes_read == 0 {
            println!();
            break;
        }
        let tokens = scanner::scan_tokens(&stdin);
        debug_tokens(&tokens);
    }
    Ok(())
}

fn debug_tokens(tokens: &[token::Token]) {
    tokens.iter().for_each(|t| println!("{}", t));
}
