use std::{net::IpAddr, sync::Arc};

use clap::Args;

use crate::{
    forwardconfig::store::ForwardConfigStore,
    forwardhandler::ForwardHandler,
    proxy,
    server,
};

#[derive(Args)]
pub struct ServerArgs {
    #[arg(long, default_value_t = String::from("127.0.0.1"))]
    target_ip: String,

    #[arg(long, default_value_t = 80)]
    target_port: u16,

    #[arg(short, long, default_value_t = 2144)]
    port: u16,

    #[arg(long, default_value_t = 2145)]
    server_port: u16,
}

pub fn handler(args: &ServerArgs) {
    let target_ip: IpAddr = args.target_ip.parse().expect("Invalid target_ip");
    log::info!("target address: {}:{}", args.target_ip, args.target_port);

    let forward_config_store = ForwardConfigStore::new();
    let forward_handler = ForwardHandler::new();
    let forward_config_store_arc = Arc::new(forward_config_store);
    let forward_handler_arc = Arc::new(forward_handler);

    let server_runtime = tokio::runtime::Runtime::new().expect("Failed to spawn server runtime");

    let server_port = args.server_port;
    let forward_config_store_server = Arc::clone(&forward_config_store_arc);
    let forward_handler_server = Arc::clone(&forward_handler_arc);
    server_runtime.spawn(async move {
        let _ = server::run(
            server_port,
            forward_config_store_server,
            forward_handler_server,
        )
        .await;
    });
    proxy::run(
        args.port,
        forward_config_store_arc,
        forward_handler_arc,
        (target_ip, args.target_port),
    );
}
