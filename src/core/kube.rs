use crate::common::fse;
use crate::core::context::Context;
use std::path::PathBuf;
use tokio::fs;

pub fn get_default_kube_filepath() -> String {
    let mut home_dir = dirs::home_dir().unwrap();
    home_dir.push(".kube/config");
    home_dir.to_string_lossy().into_owned()
}

pub async fn resolve_joycon_kube_config() -> Result<String, ()> {
    if fse::path_exists(".joycon/kube/config").await {
        let content = fse::read_file(".joycon/kube/config").await?;
        if content.trim().is_empty() {
            return Err(());
        }
        return Ok(content);
    }
    Err(())
}

pub async fn flush_joycon_kube_config(ctx: &Context) {
    match resolve_joycon_kube_config().await {
        Ok(config) => {
            let dir = PathBuf::from(&ctx.options.kube);
            fse::ensure_dir(dir.parent().unwrap()).await;
            fse::write_file(&ctx.options.kube, &config).await;
        }
        Err(_) => {
            if fse::path_exists(&ctx.options.kube).await {
                fs::remove_file(&ctx.options.kube).await.unwrap();
            }
        }
    };
}
