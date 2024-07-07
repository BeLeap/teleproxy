mod service;

use self::service::TeleproxyImpl;
use crate::{config, forwardconfig::store::ForwardConfigStore, forwardhandler::ForwardHandler, proto};
use log::info;
use std::sync::Arc;
use tonic::transport::Server;

pub async fn run(
    server_config: config::server::Server,
    forward_config_store: Arc<ForwardConfigStore>,
    forward_handler: Arc<ForwardHandler>,
) -> Result<(), Box<dyn std::error::Error>> {
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::teleproxy::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    let addr = format!("[::]:{}", server_config.server_port).parse()?;
    info!("listening port: {}", server_config.server_port);

    let svc = proto::teleproxy::teleproxy_server::TeleproxyServer::with_interceptor(
        TeleproxyImpl {
            api_key: server_config.api_key,
            forward_config_store,
            forward_handler,
        },
        interceptor,
    );

    Server::builder()
        .add_service(svc)
        .add_service(reflection_service)
        .serve(addr)
        .await?;

    Ok(())
}

fn interceptor(req: tonic::Request<()>) -> tonic::Result<tonic::Request<()>> {
    logging_interceptor(req)
}

fn logging_interceptor(req: tonic::Request<()>) -> tonic::Result<tonic::Request<()>> {
    log::trace!("request metadata {:?}", req);

    Ok(req)
}
