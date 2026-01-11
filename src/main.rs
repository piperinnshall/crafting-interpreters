use std::io::{self, Write};
use std::env; 
use std::process;
use std::fs;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        len if len > 2 => {
            eprintln!("Usage: jx [script]");
            process::exit(64);
        }
        2 => if let Err(e) = run(&args[1]) {
            eprintln!("Application error: {e}");
            process::exit(1); 
        }
        _ => run_prompt(),
    };
}

fn error(line: i32, msg: &str) {
    
}

fn report(line: i32, at: &str, msg: &str) {
    
}

fn run(path: &str) -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string(path)?;
    print!("{}", input);
    Ok(())
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
    }
}


