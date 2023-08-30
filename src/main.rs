use atty::Stream;
use colored::Colorize;
use dirs;
use serde_derive::Deserialize;
use shell_words;
use std::{
    env,
    error::Error,
    fs,
    io::{self, Write},
    path::Path,
    process::Command,
};
use whoami::username;
#[derive(Deserialize)]
struct Data {
    config: Config,
}
#[derive(Deserialize)]
struct Config {
    prompt_msg: String,
    prompt_msg_color: String,
    prompt_symbol: String,
    prompt_symbol_color: String,
    prompt_username: String,
    shell_name: String,
    execute_time: String,
    execute_time_color: String,
    git_integration: String,
    wd_color: String,
}

// TODO: documentation and cleanup
fn main() {
    if atty::is(Stream::Stdout) {
        let data = load_config();
        let prompt_msg = data.config.prompt_msg;
        let prompt_msg_color = data.config.prompt_msg_color;
        let prompt_symbol = data.config.prompt_symbol;
        let prompt_symbol_color = data.config.prompt_symbol_color;
        let prompt_username = parse_bool_config(&data.config.prompt_username);
        let shell_name = parse_bool_config(&data.config.shell_name);
        let execute_time = parse_bool_config(&data.config.execute_time);
        let execute_time_color = data.config.execute_time_color;
        let git_integration = parse_bool_config(&data.config.git_integration);
        let wd_color = data.config.wd_color;
        loop {
            // TODO: git integration
            // TODO: autocomplete
            // TODO: display username true/false
            let wd = match env::current_dir() {
                Ok(wd) => wd,
                Err(_) => panic!("Cannot determine current directory"),
            };
            println!("{}", wd.display().to_string().color(wd_color.as_str()));
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

fn load_config() -> Data {
    let home_dir = dirs::home_dir().expect("Failed to determine home directory");
    let config_path = home_dir.join(".ksh.toml");
    // TODO: update path to .ksh.toml in home_dir
    let contents = match fs::read_to_string("./.ksh.toml") {
        Ok(contents) => contents,
        Err(err) => {
            panic!("Unable to load .ksh.toml: {}", err)
        }
    };

    let data: Data = match toml::from_str(&contents) {
        Ok(config) => config,
        Err(err) => {
            panic!("Unable to read .ksh.toml: {}", err)
        }
    };

    data
}

fn parse_bool_config(value: &str) -> bool {
    match value {
        "true" => true,
        "false" => false,
        _ => true,
    }
}
