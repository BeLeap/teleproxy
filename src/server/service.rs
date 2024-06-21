use super::teleproxy_proto;
use crate::forwardconfig::store::ForwardConfigStore;
use std::{pin::Pin, sync::Arc};
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

    type ListenStream = Pin<
        Box<
            dyn tokio_stream::Stream<
                    Item = tonic::Result<teleproxy_proto::ListenResponse, tonic::Status>,
                > + Send,
        >,
    >;
    async fn listen(
        &self,
        _request: tonic::Request<tonic::Streaming<teleproxy_proto::ListenRequest>>,
    ) -> tonic::Result<tonic::Response<Self::ListenStream>> {
        todo!()
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
