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
        let a = fs::read_link(path).await.unwrap();
        let b = fs::canonicalize(".joycon/kube/config").await.unwrap();
        return a == b;
    }

    false
}

pub async fn create_joycon_kube_config_symlink(ctx: &Context) {
    let dir = PathBuf::from(&ctx.options.kube);
    let exists = fse::path_exists(".joycon/kube/config").await;
    fse::ensure_dir(dir.parent().unwrap()).await;

    if fse::path_exists(&ctx.options.kube).await {
        if exists && has_linked(&ctx.options.kube).await {
            return;
        }
        fs::remove_file(&ctx.options.kube).await.unwrap();
    }

    if exists {
        let original = fs::canonicalize(".joycon/kube/config").await.unwrap();
        symlink(original, &ctx.options.kube).unwrap();
    }
}
