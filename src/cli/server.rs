use std::net::IpAddr;

use clap::Args;
use pingora::prelude::*;

use crate::proxy::{Target, TeleProxyService};

#[derive(Args)]
pub struct ServerArgs {
    #[arg(short = 'i', long)]
    target_ip: String,

    #[arg(short = 'p', long)]
    target_port: u16,
}

pub fn handler(args: &ServerArgs) {
    let target_ip: IpAddr = args.target_ip.parse().unwrap();

    let mut proxy_server = Server::new(None).unwrap();
    proxy_server.bootstrap();

    let mut teleproxy_service = pingora_proxy::http_proxy_service(
        &proxy_server.configuration,
        TeleProxyService {
            target: Target {
                ip: target_ip,
                port: args.target_port,
            },
        },
    );
    teleproxy_service.add_tcp("0.0.0.0:6188");

    proxy_server.add_service(teleproxy_service);
    proxy_server.run_forever();
}
