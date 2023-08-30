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
    prompt: String,
    prompt_color: String,
}

// TODO: documentation
fn main() {
    if atty::is(Stream::Stdout) {
        let data = load_config();
        let prompt = data.config.prompt;
        let prompt_color = data.config.prompt_color;
        loop {
            print!("{} {} ", username(), prompt.color(prompt_color.as_str()));
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
