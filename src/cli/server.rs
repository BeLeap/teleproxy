use std::{net::IpAddr, sync::Arc};

use clap::Args;

use crate::{
    config, forwardconfig::store::ForwardConfigStore, forwardhandler::ForwardHandler, proxy, server,
};

#[derive(Args)]
pub struct ServerArgs {
    #[arg(long, short, default_value_t = String::from("/etc/teleproxy/config.yaml"))]
    config_file_path: String,

    #[arg(long)]
    target_ip: Option<String>,

    #[arg(long)]
    target_port: Option<u16>,

    #[arg(short, long)]
    port: Option<u16>,

    #[arg(short, long)]
    server_port: Option<u16>,
}

fn get_value<T, E>(
    from_arg: Option<T>,
    from_file: Result<T, E>,
    default: Option<T>,
) -> Result<T, E> {
    match from_arg {
        Some(from_arg) => Ok(from_arg),
        None => match from_file {
            Ok(from_file) => Ok(from_file),
            Err(e) => match default {
                Some(default) => Ok(default),
                None => Err(e),
            },
        },
    }
}

pub fn handler(args: &ServerArgs) {
    let server_config_from_file = config::server::Server::read(&args.config_file_path);

    let server_config = config::server::Server {
        target_ip: get_value(
            args.target_ip.clone(),
            server_config_from_file.clone().map(|it| it.target_ip),
            Some("127.0.0.1".to_string()),
        )
        .unwrap(),
        target_port: get_value(
            args.target_port,
            server_config_from_file.clone().map(|it| it.target_port),
            Some(80),
        )
        .unwrap(),
        port: get_value(
            args.port,
            server_config_from_file.clone().map(|it| it.port),
            None,
        )
        .expect("port is not set."),
        server_port: get_value(
            args.server_port,
            server_config_from_file.clone().map(|it| it.server_port),
            None,
        )
        .expect("server_port is not set."),
    };

    let target_ip: IpAddr = server_config.target_ip.parse().expect("Invalid target_ip");
    log::info!(
        "target address: {}:{}",
        server_config.target_ip,
        server_config.target_port
    );

    let forward_config_store = ForwardConfigStore::new();
    let forward_handler = ForwardHandler::new();
    let forward_config_store_arc = Arc::new(forward_config_store);
    let forward_handler_arc = Arc::new(forward_handler);

    let server_runtime = tokio::runtime::Runtime::new().expect("Failed to spawn server runtime");

    let server_port = server_config.server_port;
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
        server_config.port,
        forward_config_store_arc,
        forward_handler_arc,
        (target_ip, server_config.target_port),
    );
}
