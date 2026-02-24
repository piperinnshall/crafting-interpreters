mod scanner;
mod lexer;
mod token;

use std::{
    env,
    error::Error,
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

fn run_file(path: &str) -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string(path)?;
    run(&input)?;
    Ok(())
}

fn run_prompt() -> Result<(), Box<dyn Error>> {
    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut stdin = String::new();

        let bytes_read = io::stdin().read_line(&mut stdin)?;

        // Ctrl-D sends EOF token, should break out of loop.
        if bytes_read == 0 {
            println!();
            break;
        }

        run(&stdin)?;
    }
    Ok(())
}

fn run(source: &str) -> Result<(), String> {
    let tokens = scanner::scan_tokens(source);
    tokens.iter().for_each(|t| println!("{t}"));
    Ok(())
}
