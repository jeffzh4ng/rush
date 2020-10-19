use std::io::{self, Write};
use std::process::Command;

fn main() {
    loop {
        print!("> ");
        // need to explicitly flush this to ensure it prints before read_line
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        let mut child = Command::new(command)
            .args(args)
            .spawn()
            .unwrap();

        child.wait().unwrap();
    }
}
