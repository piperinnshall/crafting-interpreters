mod scanner;

use scanner::Scanner;
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
        _ => run_prompt(),
    };
}

fn run_prompt() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut stdin = String::new();

        let bytes_read = io::stdin()
            .read_line(&mut stdin)
            .expect("Failed to read line");

        if bytes_read == 0 {
            break;
        }

        run(&stdin);
    }
}

fn run_file(path: &str) -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string(path)?;
    run(&input);
    Ok(())
}

fn run(source: &str) {
    let scanner = Scanner::new(source);
    let tokens = scanner.as_vec();
    tokens.iter().for_each(|t| println!("{t}"));
}

fn error(line: i32, msg: &str) {
    report(line, "", msg);
}

fn report(line: i32, at: &str, msg: &str) {
    eprintln!("[line {}] Error{}: {}", line, at, msg);
}
