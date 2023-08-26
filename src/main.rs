use std::{
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
        let args: Vec<&str> = input.split(' ').collect();
        let command = args[0];
        let result = execute_system_command(&command, &args).expect("Failed to execute");
        println!("{}", result);
    }
}

fn read_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

fn execute_system_command(command: &str, args: &Vec<&str>) -> io::Result<String> {
    Ok(("placeholder".to_string()))
}
