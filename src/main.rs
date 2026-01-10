use std::env; 
use std::process;
use std::fs;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        len if len > 2 => {
            println!("Usage: jx [script]");
            process::exit(64);
        }
        2 => match run_file(&args[1]) {
            Ok(_) => process::exit(0),
            Err(err) => print_err(err)
        } ,
        _ => println!(),
    };
}

fn print_err(err: Box<dyn Error>) {
    println!("ERROR: \n{err}");
    process::exit(1);
}

fn run_file(path: &str) -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string(path)?;
    println!("{}", input);
    Ok(())
}
