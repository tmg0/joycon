use crate::common::fse;
use crate::core::hosts::get_default_hosts_filepath;
use crate::core::kube::get_default_kube_config_filepath;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Options {
    pub hosts: String,
    pub kube: String,
}

pub async fn resolve_config_filepath() -> Result<String, ()> {
    let relative_config_filepath = "./.joycon/settings.json";
    if fse::path_exists(relative_config_filepath).await {
        return Ok(relative_config_filepath.to_string());
    }
    Err(())
}

pub async fn resolve_options() -> Options {
    let default_config = Options {
        hosts: get_default_hosts_filepath(),
        kube: get_default_kube_config_filepath(),
    };

    match resolve_config_filepath().await {
        Ok(config_filepath) => {
            let content = fse::read_file(config_filepath).await.unwrap();
            let config: Options = serde_json::from_str(&content).unwrap_or_else(|_| default_config);
            config
        }
        Err(_) => default_config,
    }
}
