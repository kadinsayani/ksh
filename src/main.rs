use std::{
    io::{self, Write},
    process::Command,
};

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");
        let input = read_input().expect("Failed to read input");
        let result = execute(&input).expect("Failed to execute");
        println!("{}", result);
    }
}

fn read_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

fn execute(input: &str) -> io::Result<String> {
    match input {
        "cd" => Ok(format!("cd")),
        _ => Ok(format!("Unknown command: {}", input)),
    }
}
