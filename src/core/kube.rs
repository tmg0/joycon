use crate::common::fse;
use crate::core::context::Context;
use std::os::unix::fs::symlink;
use std::path::PathBuf;
use tokio::fs;

pub fn get_default_kube_config_filepath() -> String {
    let mut home_dir = dirs::home_dir().unwrap();
    home_dir.push(".kube/config");
    home_dir.to_string_lossy().into_owned()
}

async fn is_symlink(path: &String) -> bool {
    fs::symlink_metadata(path).await.is_ok()
}

async fn has_linked(path: &String) -> bool {
    if is_symlink(path).await {
        if fs::read_link(path).await.unwrap().to_string_lossy() == ".joycon/kube/config" {
            return true;
        }
        return false;
    }

    false
}

pub async fn create_joycon_kube_config_symlink(ctx: &Context) {
    let dir = PathBuf::from(&ctx.options.kube);
    fse::ensure_dir(dir.parent().unwrap()).await;

    if fse::path_exists(&ctx.options.kube).await {
        if has_linked(&ctx.options.kube).await {
            return;
        } else {
            fs::remove_file(&ctx.options.kube).await.unwrap();
        }
    }

    if fse::path_exists(".joycon/kube/config").await {
        symlink(".joycon/kube/config", &ctx.options.kube).unwrap();
    }
}
