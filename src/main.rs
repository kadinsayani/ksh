mod config;

use atty::Stream;
use colored::Colorize;
use shell_words;
use std::{
    env,
    error::Error,
    io::{self, Write},
    path::Path,
    process::Command,
};
use whoami::username;

// TODO: documentation and cleanup
fn main() {
    if atty::is(Stream::Stdout) {
        let data = config::load_config();
        let prompt_msg = data.config.prompt_msg;
        let prompt_msg_color = data.config.prompt_msg_color;
        let prompt_symbol = data.config.prompt_symbol;
        let prompt_symbol_color = data.config.prompt_symbol_color;
        let prompt_username = config::parse_bool_config(&data.config.prompt_username);
        let shell_name = config::parse_bool_config(&data.config.shell_name);
        // TODO
        let execute_time = config::parse_bool_config(&data.config.execute_time);
        let execute_time_color = data.config.execute_time_color;
        // TODO
        let git_integration = config::parse_bool_config(&data.config.git_integration);
        let git_integration_color = data.config.git_integration_color;
        let mut git_branch: String = String::from("");
        if git_integration {
            // get current git branch
            // git rev-parse --abbrev-ref HEAD
            let output = Command::new("git")
                .arg("rev-parse")
                .arg("--abbrev-ref")
                .arg("HEAD")
                .output()
                .expect("Failed to execute command");

            if output.status.success() {
                git_branch = String::from_utf8_lossy(&output.stdout).to_string()
            }
        }
        let wd_color = data.config.wd_color;
        loop {
            // TODO: autocomplete
            let wd = match env::current_dir() {
                Ok(wd) => wd,
                Err(_) => panic!("Cannot determine current directory"),
            };
            print!("{}", wd.display().to_string().color(wd_color.as_str()));
            println!(" {}", git_branch.color(git_integration_color.as_str()));
            let mut prompt: String = prompt_msg.clone();
            if !prompt_msg.is_empty() && shell_name {
                prompt.push_str(" | ");
            }
            if !prompt_msg.is_empty() && prompt_username {
                prompt.push_str(" | ");
            }
            if shell_name {
                prompt.push_str("ksh");
            }
            if shell_name && prompt_username {
                prompt.push_str(" | ");
            }
            if prompt_username {
                prompt.push_str(username().as_str());
            };
            print!(
                "{} {} ",
                prompt.color(prompt_msg_color.as_str()),
                prompt_symbol.color(prompt_symbol_color.as_str())
            );
            io::stdout().flush().expect("Failed to flush stdout");
            let input = match read_input() {
                Ok(input) => input,
                Err(err) => {
                    eprintln!("{}", err);
                    continue;
                }
            };
            let tokens = match shell_words::split(&input) {
                Ok(tokens) => tokens,
                Err(err) => {
                    eprintln!("{}", err);
                    continue;
                }
            };
            let command = &tokens[0];
            let args = &tokens[1..];
            match execute_system_command(&command, &args) {
                Ok(result) => println!("{}", result),
                Err(err) => eprintln!("{:?}", err),
            }
        }
    }
}

fn read_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

fn execute_system_command(command: &str, args: &[String]) -> Result<String, Box<dyn Error>> {
    match command {
        "cd" => {
            let path = Path::new(&args[0]);
            match env::set_current_dir(&path) {
                Ok(()) => Ok('\n'.to_string()),
                Err(err) => Err(format!("Error: {}", err).into()),
            }
        }
        command => {
            let output = Command::new(command)
                .args(args)
                .output()
                .expect("Failed to execute command");

            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                Ok(String::from_utf8_lossy(&output.stderr).to_string())
            }
        }
    }
}
