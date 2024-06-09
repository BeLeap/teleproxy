mod service;

use std::sync::Arc;

use tonic::transport::Server;

use crate::forwardconfig::store::ForwardConfigStore;

use self::service::TeleproxyImpl;

pub mod teleproxy_proto {
    tonic::include_proto!("teleproxy");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("teleproxy_descriptor");
}

pub async fn run(
    port: u16,
    _config: Arc<ForwardConfigStore>,
) -> Result<(), Box<dyn std::error::Error>> {
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(teleproxy_proto::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    let addr = format!("[::1]:{}", port).parse()?;
    println!("listening on {}", addr);

    Server::builder()
        .add_service(teleproxy_proto::teleproxy_server::TeleproxyServer::new(
            TeleproxyImpl::default(),
        ))
        .add_service(reflection_service)
        .serve(addr)
        .await?;

    Ok(())
}
