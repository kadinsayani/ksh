use std::{
    io::{self, Write},
    process::{Command, Output},
};

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");
        let input = read_input().expect("Failed to read input");
        let mut args: Vec<&str> = input.split(' ').collect();
        let command = args[0];
        args.remove(0);
        let result = execute_system_command(&command, &args);
        println!("{}", result);
    }
}

fn read_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

fn execute_system_command(command: &str, args: &Vec<&str>) -> String {
    let output: Output;
    match command {
        "cd" => "cd".to_string(),
        command => {
            let output = Command::new(command)
                .args(args)
                .output()
                .expect("Failed to execute command");

            if output.status.success() {
                String::from_utf8_lossy(&output.stdout).to_string()
            } else {
                String::from_utf8_lossy(&output.stderr).to_string()
            }
        }
    }
}
