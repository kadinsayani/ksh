use std::{
    collections::HashMap,
    io::{self, Write},
    process::Command,
};

fn main() {
    let mut command_processors: HashMap<&str, Box<dyn Fn(&str) -> io::Result<String>>> =
        HashMap::new();
    command_processors.insert("cd", Box::new(|_args| Ok(format!("cd"))));

    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");
        let input = read_input().expect("Failed to read input");
        let result = execute(&input, &command_processors).expect("Failed to execute");
        println!("{}", result);
    }
}

fn read_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

type CommandProcessor = Box<dyn Fn(&str) -> io::Result<String>>;

fn execute(
    input: &str,
    command_processors: &HashMap<&str, CommandProcessor>,
) -> io::Result<String> {
    if let Some(processor) = command_processors.get(input) {
        processor(input)
    } else {
        Ok(format!("Unknown command: {}", input))
    }
}
