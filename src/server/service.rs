use std::sync::Arc;

use crate::forwardconfig::store::ForwardConfigStore;

use super::teleproxy_proto;

use log::trace;
use tokio_stream::wrappers::ReceiverStream;

pub struct TeleproxyImpl {
    forward_config_store: Arc<ForwardConfigStore>
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
        trace!("gRPC Health Request Received");
        let response = teleproxy_proto::EchoResponse {};
        Ok(tonic::Response::new(response))
    }

    async fn register(
        &self,
        _request: tonic::Request<teleproxy_proto::RegisterRequest>,
    ) -> tonic::Result<tonic::Response<teleproxy_proto::RegisterResponse>> {
        unimplemented!()
    }

    type ListenStream = ReceiverStream<tonic::Result<teleproxy_proto::ListenResponse>>;
    async fn listen(
        &self,
        _request: tonic::Request<tonic::Streaming<teleproxy_proto::ListenRequest>>,
    ) -> tonic::Result<tonic::Response<Self::ListenStream>> {
        unimplemented!()
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
        unimplemented!()
    }

    async fn flush(
        &self,
        _request: tonic::Request<teleproxy_proto::FlushRequest>,
    ) -> tonic::Result<tonic::Response<teleproxy_proto::FlushResponse>> {
        unimplemented!()
    }
}
