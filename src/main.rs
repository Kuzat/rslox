use std::{
    fs,
    io::{self, Write},
};

use clap::Parser;
use rslox::{error::LoxError, lexer::Scanner};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    file: Option<String>,

    #[arg(short, long)]
    repl: bool,
}

fn main() {
    let args = Args::parse();

    // Alterantively run repl if no flag is passed
    if args.repl {
        run_prompt();
        return;
    } else if let Some(file_name) = args.file {
        run_file(file_name);
        return;
    } else {
        println!("Error: No file or REPL flag passed");
    }
}

fn run_prompt() {
    // Loop and ask the users for input
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");

        let result = run(input);
        if let Err(e) = result {
            e.report();
        }
        println!("");
    }
}

fn run_file(file_name: String) {
    // read file name as string
    let file_string = fs::read_to_string(file_name).expect("Error reading file");

    if let Err(e) = run(file_string) {
        e.report();
    }
}

fn run(source: String) -> Result<(), LoxError> {
    let mut lexer = Scanner::new(source);
    let tokens = lexer.scan_tokens()?;

    for token in tokens {
        println!("{}", token);
    }

    Ok(())
}
