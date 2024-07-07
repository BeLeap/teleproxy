use crate::{
    dto::{self, header::Header, phase::ListenPhase},
    forwardconfig::store::ForwardConfigStore,
    forwardhandler::ForwardHandler, proto,
};
use std::{pin::Pin, sync::Arc};
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::Status;

pub struct TeleproxyImpl {
    pub forward_config_store: Arc<ForwardConfigStore>,
    pub forward_handler: Arc<ForwardHandler>,
}

#[tonic::async_trait]
impl proto::teleproxy::teleproxy_server::Teleproxy for TeleproxyImpl {
    async fn health(
        &self,
        request: tonic::Request<proto::teleproxy::EchoRequest>,
    ) -> tonic::Result<tonic::Response<proto::teleproxy::EchoResponse>> {
        let request = request.into_inner();
        log::trace!("health requested with payload {:?}", request);

        let response = proto::teleproxy::EchoResponse {};
        Ok(tonic::Response::new(response))
    }

    async fn register(
        &self,
        request: tonic::Request<proto::teleproxy::RegisterRequest>,
    ) -> tonic::Result<tonic::Response<proto::teleproxy::RegisterResponse>> {
        let request = request.into_inner();
        log::trace!("register requested with payload {:?}", request);

        let id = ulid::Ulid::new().to_string();

        self.forward_config_store
            .insert(Header {
                key: request.header_key, 
                value: request.header_value,
            }, &id);

        Ok(tonic::Response::new(proto::teleproxy::RegisterResponse { id }))
    }

    type ListenStream = Pin<
        Box<
            dyn tokio_stream::Stream<
                    Item = tonic::Result<proto::teleproxy::ListenResponse, tonic::Status>,
                > + Send,
        >,
    >;
    async fn listen(
        &self,
        request: tonic::Request<tonic::Streaming<proto::teleproxy::ListenRequest>>,
    ) -> tonic::Result<tonic::Response<Self::ListenStream>> {
        let mut in_stream = request.into_inner();

        let id = match in_stream.next().await {
            Some(result) => match result {
                Ok(request) => {
                    if request.phase == ListenPhase::Init as i32 {
                        request.id
                    } else {
                        panic!("First request for listen must be INIT")
                    }
                },
                Err(e) => {
                    log::error!("{:?}", e);
                    panic!("{:?}", e);
                }
            },
            None => {
                log::error!("Failed to get next request");
                panic!("");
            }
        };
        let (tx, mut rx) = tokio::sync::mpsc::channel::<(
            dto::Request,
            tokio::sync::oneshot::Sender<dto::Response>,
        )>(128);
        self.forward_handler.register_sender(&id, tx);

        let (stream_tx, steram_rx) = tokio::sync::mpsc::channel(128);
        let out_stream = ReceiverStream::new(steram_rx);

        tokio::spawn(async move {
            while let Some((request, response_tx)) = rx.recv().await {
                let headers = request
                    .headers
                    .iter()
                    .map(|header| {
                        (
                            header.key.clone(),
                            header.value.clone(),
                        )
                    })
                    .collect();

                let response_result = stream_tx
                    .send(Ok(proto::teleproxy::ListenResponse {
                        phase: ListenPhase::Tunneling as i32,
                        method: request.method,
                        url: request.uri,
                        headers,
                        body: request.body,
                    }))
                    .await;
                match response_result {
                    Ok(_) => {}
                    Err(_) => todo!(),
                }

                let mut response: Option<dto::Response> = None;
                while let Some(result) = in_stream.next().await {
                    if let Ok(request) = result {
                        response = Some(dto::Response::from_pb(request));
                        break;
                    }
                }

                let response_forward_result = response_tx.send(response.unwrap());
                match response_forward_result {
                    Ok(_) => todo!(),
                    Err(_) => todo!(),
                }
            }
        });

        Ok(tonic::Response::new(
            Box::pin(out_stream) as Self::ListenStream
        ))
    }

    async fn deregister(
        &self,
        request: tonic::Request<proto::teleproxy::DeregisterRequest>,
    ) -> tonic::Result<tonic::Response<proto::teleproxy::DeregisterResponse>> {
        let request = request.into_inner();
        log::trace!("deregister requested with payload {:?}", request);

        self.forward_config_store.remove_by_id(&request.id);

        Ok(tonic::Response::new(proto::teleproxy::DeregisterResponse {}))
    }

    async fn dump(
        &self,
        request: tonic::Request<proto::teleproxy::DumpRequest>,
    ) -> Result<tonic::Response<proto::teleproxy::DumpResponse>, tonic::Status> {
        let request = request.into_inner();
        log::trace!("dump requested with payload {:?}", request);

        let forward_config_store = Arc::clone(&self.forward_config_store);
        let config_dump = serde_yml::to_string(&forward_config_store.list());

        match config_dump {
            Ok(config_dump) => Ok(tonic::Response::new(proto::teleproxy::DumpResponse {
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
        request: tonic::Request<proto::teleproxy::FlushRequest>,
    ) -> tonic::Result<tonic::Response<proto::teleproxy::FlushResponse>> {
        let request = request.into_inner();
        log::trace!("flush requested with payload {:?}", request);

        unimplemented!()
    }
}
