use std::{env, sync::Arc};

use clap::Args;

use crate::{
    config, forwardconfig::store::ForwardConfigStore, forwardhandler::ForwardHandler, proxy, server,
};

#[derive(Args)]
pub struct ServerArgs {
    #[arg(long, short, default_value_t = String::from("/etc/teleproxy/config.yaml"))]
    config_file_path: String,

    #[arg(long)]
    api_key: Option<String>,

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
    from_env: Option<T>,
    from_file: Result<T, E>,
    default: Option<T>,
) -> Result<T, E> {
    match from_arg {
        Some(from_arg) => Ok(from_arg),
        None => match from_env {
            Some(from_env) => Ok(from_env),
            None => match from_file {
                Ok(from_file) => Ok(from_file),
                Err(e) => match default {
                    Some(default) => Ok(default),
                    None => Err(e),
                },
            },
        },
    }
}

pub fn handler(args: &ServerArgs) {
    let server_config_from_file = config::server::Server::read(&args.config_file_path);

    let server_config = config::server::Server {
        target_ip: get_value(
            args.target_ip.clone(),
            env::var("TARGET_IP").ok(),
            server_config_from_file.clone().map(|it| it.target_ip),
            Some("127.0.0.1".to_string()),
        )
        .unwrap(),
        target_port: get_value(
            args.target_port,
            env::var("TARGET_PORT")
                .ok()
                .and_then(|target_port| target_port.parse::<u16>().ok()),
            server_config_from_file.clone().map(|it| it.target_port),
            Some(80),
        )
        .unwrap(),
        port: get_value(
            args.port,
            env::var("PORT")
                .ok()
                .and_then(|target_port| target_port.parse::<u16>().ok()),
            server_config_from_file.clone().map(|it| it.port),
            None,
        )
        .expect("port is not set."),
        server_port: get_value(
            args.server_port,
            env::var("SERVER_PORT")
                .ok()
                .and_then(|target_port| target_port.parse::<u16>().ok()),
            server_config_from_file.clone().map(|it| it.server_port),
            None,
        )
        .expect("server_port is not set."),
        api_key: get_value(
            args.api_key.clone(),
            env::var("API_KEY").ok(),
            server_config_from_file.clone().map(|it| it.api_key),
            None,
        )
        .expect("api_key is not set."),
    };

    let forward_config_store = ForwardConfigStore::new();
    let forward_handler = ForwardHandler::new();
    let forward_config_store_arc = Arc::new(forward_config_store);
    let forward_handler_arc = Arc::new(forward_handler);

    let server_runtime = tokio::runtime::Runtime::new().expect("Failed to spawn server runtime");

    let server_config_server = server_config.clone();
    let forward_config_store_server = Arc::clone(&forward_config_store_arc);
    let forward_handler_server = Arc::clone(&forward_handler_arc);
    server_runtime.spawn(async move {
        let _ = server::run(
            server_config_server,
            forward_config_store_server,
            forward_handler_server,
        )
        .await;
    });
    proxy::run(
        server_config,
        forward_config_store_arc,
        forward_handler_arc,
    );
}
