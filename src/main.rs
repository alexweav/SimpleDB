mod row;
mod statement;

use std::io;
use std::io::Write;
use std::process;

use crate::statement::Statement;

/// Entry point for interactive mode.
fn main() {
    println!("Welcome to SimpleDB!");
    loop {
        print_prompt();
        match read_input() {
            Ok(value) => {
                let text = value.trim();
                if text.chars().next() == Some('.') {
                    handle_meta_command(text);
                }
                match Statement::parse(text) {
                    Ok(statement) => statement.execute(),
                    Err(err) => eprintln!("{}", err),
                }
            }
            Err(err) => {
                eprintln!("error: {:?}", err);
                process::exit(1);
            }
        };
    }
}

/// Metacommand handler for interactive mode.
fn handle_meta_command(input: &str) {
    if input == ".exit" {
        println!("bye");
        process::exit(0);
    } else {
        eprintln!("Unrecognized command {:?}", input);
    }
}

/// Reads a command, including whitespace.
fn read_input() -> Result<String, io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}

/// Prints the interactive mode prompt.
fn print_prompt() {
    print!("db > ");
    io::stdout().flush().unwrap();
}
