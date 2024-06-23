pub mod teleproxy_pb {
    tonic::include_proto!("teleproxy");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("teleproxy_descriptor");
}
mod service;

use self::service::TeleproxyImpl;
use crate::{forwardconfig::store::ForwardConfigStore, forwardhandler::ForwardHandler};
use log::info;
use std::sync::Arc;
use tonic::transport::Server;

pub async fn run(
    port: u16,
    forward_config_store: Arc<ForwardConfigStore>,
    forward_handler: Arc<ForwardHandler>,
) -> Result<(), Box<dyn std::error::Error>> {
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(teleproxy_pb::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    let addr = format!("[::]:{}", port).parse()?;
    info!("listening port: {}", port);

    let svc = teleproxy_pb::teleproxy_server::TeleproxyServer::with_interceptor(
        TeleproxyImpl {
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
