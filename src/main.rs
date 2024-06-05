mod commands;
mod hosts;
mod utils;

#[tokio::main]
async fn main() {
    let config = utils::resolve_config().await;
    println!("{}", config.hosts);
}
