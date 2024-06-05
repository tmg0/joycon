use dirs;
use serde::{Deserialize, Serialize};
use tokio::fs::read_to_string;

use super::fse::{ensure_file, path_exists};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_hosts")]
    pub hosts: String,
}

fn default_hosts() -> String {
    "/etc/hosts".to_string()
}

pub async fn resolve_config_filepath() -> String {
    let relative_config_filepath = "./.joycon/config.json";
    if path_exists(relative_config_filepath).await {
        return relative_config_filepath.to_string();
    }
    let mut home_dir = dirs::home_dir().unwrap();
    home_dir.push(relative_config_filepath);
    let absolute_config_filepath = home_dir.to_string_lossy().into_owned();
    if path_exists(absolute_config_filepath.clone()).await {
        return absolute_config_filepath;
    }
    ensure_file(relative_config_filepath).await;
    relative_config_filepath.to_string()
}

pub async fn resolve_config() -> Config {
    let config_filepath = resolve_config_filepath().await;
    let content = read_to_string(config_filepath).await.unwrap();
    let config: Config = serde_json::from_str(&content).unwrap();
    config
}
