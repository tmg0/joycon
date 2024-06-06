mod common;
mod core;
mod hosts;

#[tokio::main]
async fn main() {
    let options = core::options::resolve_options().await;
    let context = core::context::create_context(options);
    println!("{}", hosts::insert_joycon_hosts(context).await);
}
