use std::io::{self, Write};
use std::process::Command;
use std::path::Path;
use std::env;

fn main() {
    let mut history: Vec<String> = Vec::new();
    
    loop {
        print!("> ");
        // need to explicitly flush this to ensure it prints before read_line
        io::stdout().flush().unwrap();
        
        // parsing
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        history.push(command.to_string());

        match command {
            "cd" => {
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);

                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            },
            "history" => {
                for (i, command) in history.iter().enumerate() {
                    println!("{} {}", i, command);
                }
            },
            "exit" => return,
            command => {
                let child = Command::new(command)
                    .args(args)
                    .spawn();

                match child {
                    Ok(mut child) => { child.wait().unwrap(); },
                    Err(e) => eprintln!("{}", e),
                }
            }
        }
    }
}