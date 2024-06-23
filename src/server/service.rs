use super::teleproxy_proto;
use crate::forwardconfig::{header::Header, store::ForwardConfigStore};
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
        request: tonic::Request<teleproxy_proto::EchoRequest>,
    ) -> tonic::Result<tonic::Response<teleproxy_proto::EchoResponse>> {
        let request = request.into_inner();
        log::trace!("health requested with payload {:?}", request);

        let response = teleproxy_proto::EchoResponse {};
        Ok(tonic::Response::new(response))
    }

    async fn register(
        &self,
        request: tonic::Request<teleproxy_proto::RegisterRequest>,
    ) -> tonic::Result<tonic::Response<teleproxy_proto::RegisterResponse>> {
        let request = request.into_inner();
        log::trace!("register requested with payload {:?}", request);

        let id = ulid::Ulid::new().to_string();

        self.forward_config_store
            .insert(Header::new(request.header_key, request.header_value), &id);

        Ok(tonic::Response::new(teleproxy_proto::RegisterResponse {
            id,
        }))
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
        request: tonic::Request<teleproxy_proto::DeregisterRequest>,
    ) -> tonic::Result<tonic::Response<teleproxy_proto::DeregisterResponse>> {
        let request = request.into_inner();
        log::trace!("deregister requested with payload {:?}", request);

        self.forward_config_store.remove_by_id(&request.id);

        Ok(tonic::Response::new(teleproxy_proto::DeregisterResponse {}))
    }

    async fn dump(
        &self,
        request: tonic::Request<teleproxy_proto::DumpRequest>,
    ) -> Result<tonic::Response<teleproxy_proto::DumpResponse>, tonic::Status> {
        let request = request.into_inner();
        log::trace!("dump requested with payload {:?}", request);

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
        request: tonic::Request<teleproxy_proto::FlushRequest>,
    ) -> tonic::Result<tonic::Response<teleproxy_proto::FlushResponse>> {
        let request = request.into_inner();
        log::trace!("flush requested with payload {:?}", request);

        unimplemented!()
    }
}
