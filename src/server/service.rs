use std::{collections::HashMap, pin::Pin, sync::Arc};

use crate::forwardconfig::store::ForwardConfigStore;

use super::teleproxy_proto;

use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::Status;

pub struct TeleproxyImpl {
    forward_config_store: Arc<ForwardConfigStore>,
}

impl TeleproxyImpl {
    pub fn new(forward_config_store: Arc<ForwardConfigStore>) -> Self {
        Self {
            forward_config_store,
        }
    }
}

#[tonic::async_trait]
impl teleproxy_proto::teleproxy_server::Teleproxy for TeleproxyImpl {
    async fn health(
        &self,
        _request: tonic::Request<teleproxy_proto::EchoRequest>,
    ) -> tonic::Result<tonic::Response<teleproxy_proto::EchoResponse>> {
        log::trace!("gRPC Health Request Received");
        let response = teleproxy_proto::EchoResponse {};
        Ok(tonic::Response::new(response))
    }

    async fn register(
        &self,
        _request: tonic::Request<teleproxy_proto::RegisterRequest>,
    ) -> tonic::Result<tonic::Response<teleproxy_proto::RegisterResponse>> {
        unimplemented!()
    }

    type ListenStream = Pin<Box<dyn tokio_stream::Stream<Item = tonic::Result<teleproxy_proto::ListenResponse, tonic::Status>> + Send>>;
    async fn listen(
        &self,
        request: tonic::Request<tonic::Streaming<teleproxy_proto::ListenRequest>>,
    ) -> tonic::Result<tonic::Response<Self::ListenStream>> {
        let mut in_stream = request.into_inner();
        let (tx, rx) = tokio::sync::mpsc::channel(128);

        tokio::spawn(async move {
            while let Some(result) = in_stream.next().await {
                match result {
                    Ok(_v) => tx
                        .send(Ok(teleproxy_proto::ListenResponse {
                            method: "".to_string(),
                            url: "".to_string(),
                            header: HashMap::new(),
                            body: Vec::new(),
                        }))
                        .await
                        .expect(""),
                    Err(err) => match tx.send(Err(err)).await {
                        Ok(_) => (),
                        Err(_err) => break,
                    },
                }
            }
        });

        let out_stream = ReceiverStream::new(rx);
        Ok(tonic::Response::new(
            Box::pin(out_stream) as Self::ListenStream
        ))
    }

    async fn deregister(
        &self,
        _request: tonic::Request<teleproxy_proto::DeregisterRequest>,
    ) -> tonic::Result<tonic::Response<teleproxy_proto::DeregisterResponse>> {
        unimplemented!()
    }

    async fn dump(
        &self,
        _request: tonic::Request<teleproxy_proto::DumpRequest>,
    ) -> Result<tonic::Response<teleproxy_proto::DumpResponse>, tonic::Status> {
        let forward_config_store = Arc::clone(&self.forward_config_store);
        let config_dump = serde_yml::to_string(&forward_config_store.list());

        match config_dump {
            Ok(config_dump) => Ok(tonic::Response::new(teleproxy_proto::DumpResponse {
                dump: config_dump,
            })),
            Err(err) => {
                log::error!("failed to dump config: {err}");
                Err(Status::internal("failed to dump config"))
            }
        }
    }

    async fn flush(
        &self,
        _request: tonic::Request<teleproxy_proto::FlushRequest>,
    ) -> tonic::Result<tonic::Response<teleproxy_proto::FlushResponse>> {
        unimplemented!()
    }
}
