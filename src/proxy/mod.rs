use std::{net::IpAddr, sync::Arc};

use pingora::server::Server;

use crate::{config, forwardconfig::store::ForwardConfigStore, forwardhandler::ForwardHandler};

pub mod service;

pub fn run(
    server_config: config::server::Server,
    forward_config_store: Arc<ForwardConfigStore>,
    forward_handler: Arc<ForwardHandler>,
) {
    let target_ip: IpAddr = server_config.target_ip.parse().expect("Invalid target_ip");
    tracing::info!(
        "target address: {}:{}",
        server_config.target_ip,
        server_config.target_port
    );

    let mut proxy_server = Server::new(None).unwrap();
    proxy_server.bootstrap();

    let mut teleproxy_service = pingora_proxy::http_proxy_service(
        &proxy_server.configuration,
        service::TeleproxyPingoraService {
            forward_config_store,
            forward_handler,
            target: (target_ip, server_config.target_port),
        },
    );
    teleproxy_service.add_tcp(&format!("0.0.0.0:{}", server_config.port).to_string());
    tracing::info!("listening port: {}", server_config.port);

    proxy_server.add_service(teleproxy_service);
    proxy_server.run_forever();
}
