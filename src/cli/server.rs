use std::{net::IpAddr, sync::Arc};

use clap::Args;
use pingora::prelude::*;

use crate::{forwardconfig::store::ForwardConfigStore, proxy::{Target, TeleproxyService}, server};

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

    let forward_config_store = ForwardConfigStore::new();
    let forward_config_store_arc = Arc::new(forward_config_store);

    let server_runtime = tokio::runtime::Runtime::new().expect("Failed to spawn server runtime");

    let server_port = args.server_port;
    let forward_config_store_server = Arc::clone(&forward_config_store_arc);
    server_runtime.spawn(async move {
        server::run(server_port, forward_config_store_server)
    });
    let mut proxy_server = Server::new(None).unwrap();
    proxy_server.bootstrap();

    let teleproxy_service = TeleproxyService::new(forward_config_store_arc, Target { ip: target_ip, port: args.target_port });

    let mut teleproxy_service = pingora_proxy::http_proxy_service(
        &proxy_server.configuration,
        teleproxy_service,
    );
    teleproxy_service.add_tcp(&format!("0.0.0.0:{}", args.port).to_string());

    proxy_server.add_service(teleproxy_service);
    proxy_server.run_forever();
}
