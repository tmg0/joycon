mod commands;
mod common;
mod hosts;

#[tokio::main]
async fn main() {
    let config = common::config::resolve_config().await;
    println!("{}", config.hosts);
}
