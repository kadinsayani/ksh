use dirs;
use serde_derive::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Data {
    pub config: Config,
}
#[derive(Deserialize)]
pub struct Config {
    pub prompt_msg: String,
    pub prompt_msg_color: String,
    pub prompt_symbol: String,
    pub prompt_symbol_color: String,
    pub prompt_username: String,
    pub shell_name: String,
    pub execute_time: String,
    pub execute_time_color: String,
    pub git_integration: String,
    pub git_integration_color: String,
    pub cwd_color: String,
}

pub fn load_config() -> Data {
    let home_dir = dirs::home_dir().expect("Failed to determine home directory");
    let config_path = home_dir.join(".ksh.toml");
    // TODO: update path to .kshrc in home_dir
    let contents = match fs::read_to_string("./.kshrc") {
        Ok(contents) => contents,
        Err(err) => {
            panic!("Unable to load .kshrc: {}", err)
        }
    };

    let data: Data = match toml::from_str(&contents) {
        Ok(config) => config,
        Err(err) => {
            panic!("Unable to read .kshrc: {}", err)
        }
    };

    data
}

pub fn parse_bool_config(value: &str) -> bool {
    match value {
        "true" => true,
        "false" => false,
        _ => true,
    }
}
