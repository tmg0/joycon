use dns::flush_dns_cache;

use crate::{common::fse, core::context::Context};
use std::env::consts::OS;
pub mod dns;

pub fn get_default_hosts_filepath() -> String {
    if OS == "windows" {
        return "/Windows/System32/dirvers/etc/hosts".to_string();
    }

    "/etc/hosts".to_string()
}

pub async fn resolve_joycon_hosts() -> Result<String, ()> {
    if fse::path_exists(".joycon/hosts").await {
        let content = fse::read_file(".joycon/hosts").await.unwrap();
        if content.trim().is_empty() {
            return Err(());
        }
        return Ok(content);
    }
    Err(())
}

pub async fn insert_joycon_hosts(ctx: &Context) -> String {
    let origin_hosts = fse::read_file(&ctx.options.hosts).await.unwrap();
    let index = origin_hosts.find("# GENERATED BY JOYCON");

    let static_hosts = match index {
        Some(i) => &origin_hosts[..i],
        None => &origin_hosts,
    };

    let hosts = match resolve_joycon_hosts().await {
        Ok(joycon_hosts) => format!("{}\n# GENERATED BY JOYCON\n{}", static_hosts, joycon_hosts),
        Err(_) => static_hosts.to_string(),
    };

    if origin_hosts != hosts {
        fse::write_file(&ctx.options.hosts, &hosts.trim()).await;
        flush_dns_cache();
    }

    hosts
}
