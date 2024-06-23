use std::{net::IpAddr, sync::Arc};

use pingora::server::Server;

use crate::{forwardconfig::store::ForwardConfigStore, forwardhandler::ForwardHandler};

pub mod service;

pub fn run(
    port: u16,
    forward_config_store: Arc<ForwardConfigStore>,
    forward_handler: Arc<ForwardHandler>,
    target: (IpAddr, u16),
) {
    let mut proxy_server = Server::new(None).unwrap();
    proxy_server.bootstrap();

    let mut teleproxy_service = pingora_proxy::http_proxy_service(
        &proxy_server.configuration,
        service::TeleproxyPingoraService {
            forward_config_store,
            forward_handler,
            target,
        },
    );
    teleproxy_service.add_tcp(&format!("0.0.0.0:{}", port).to_string());
    log::info!("listening port: {}", port);

    proxy_server.add_service(teleproxy_service);
    proxy_server.run_forever();
}
