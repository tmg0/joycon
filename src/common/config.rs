use dirs;
use serde::{Deserialize, Serialize};
use std::fs;
use tokio::fs::read_to_string;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub hosts: String,
}

pub fn resolve_config_filepath() -> String {
    let relative_config_filepath = "./.joycon/config.json";
    if fs::metadata(relative_config_filepath).is_ok() {
        return relative_config_filepath.to_string();
    }
    let mut absolute_config_filepath = dirs::home_dir().unwrap();
    absolute_config_filepath.push(relative_config_filepath);
    absolute_config_filepath.to_string_lossy().into_owned()
}

pub async fn resolve_config() -> Config {
    let config_filepath = resolve_config_filepath();
    let content = read_to_string(config_filepath).await.unwrap();
    let config: Config = serde_json::from_str(&content).unwrap();
    config
}
