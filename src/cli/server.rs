use clap::Args;
use pingora::prelude::*;

use crate::proxy::TeleProxyService;

#[derive(Args)]
pub struct ServerArgs {
    #[arg(short, long)]
    target: String,
}

pub fn handler(args: &ServerArgs) {
    let mut proxy_server = Server::new(None).unwrap();
    proxy_server.bootstrap();

    let teleproxy_service = pingora_proxy::http_proxy_service(
        &proxy_server.configuration,
        TeleProxyService {
            target: args.target.clone(),
        },
    );

    proxy_server.add_service(teleproxy_service);
    proxy_server.run_forever();
}
