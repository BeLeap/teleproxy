mod service;

use std::sync::Arc;

use tonic::transport::Server;

use crate::forwardconfig::store::ForwardConfigStore;

use self::service::TeleproxyImpl;

pub mod teleproxy_proto {
    tonic::include_proto!("teleproxy");
}

pub async fn run(
    port: u16,
    _config: Arc<ForwardConfigStore>,
) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("[::1]:{}", port).parse()?;

    Server::builder()
        .add_service(teleproxy_proto::teleproxy_server::TeleproxyServer::new(
            TeleproxyImpl::default(),
        ))
        .serve(addr)
        .await?;

    Ok(())
}
