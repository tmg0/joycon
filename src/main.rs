mod common;
mod core;

#[tokio::main]
async fn main() {
    let options = core::options::resolve_options().await;
    let context = core::context::create_context(options);
    core::hosts::insert_joycon_hosts(&context).await;
    core::kube::flush_joycon_kube_config(&context).await;
}
