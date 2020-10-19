use std::io;
use std::process::Command;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let command = input.trim();

    Command::new(command)
        .spawn()
        .unwrap();
}
