use super::teleproxy_pb;
use crate::{
    dto,
    forwardconfig::{header::Header, store::ForwardConfigStore},
    forwardhandler::ForwardHandler,
};
use std::{pin::Pin, sync::Arc};
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::Status;

pub struct TeleproxyImpl {
    pub forward_config_store: Arc<ForwardConfigStore>,
    pub forward_handler: Arc<ForwardHandler>,
}

#[tonic::async_trait]
impl teleproxy_pb::teleproxy_server::Teleproxy for TeleproxyImpl {
    async fn health(
        &self,
        request: tonic::Request<teleproxy_pb::EchoRequest>,
    ) -> tonic::Result<tonic::Response<teleproxy_pb::EchoResponse>> {
        let request = request.into_inner();
        log::trace!("health requested with payload {:?}", request);

        let response = teleproxy_pb::EchoResponse {};
        Ok(tonic::Response::new(response))
    }

    async fn register(
        &self,
        request: tonic::Request<teleproxy_pb::RegisterRequest>,
    ) -> tonic::Result<tonic::Response<teleproxy_pb::RegisterResponse>> {
        let request = request.into_inner();
        log::trace!("register requested with payload {:?}", request);

        let id = ulid::Ulid::new().to_string();

        self.forward_config_store
            .insert(Header::new(request.header_key, request.header_value), &id);

        Ok(tonic::Response::new(teleproxy_pb::RegisterResponse { id }))
    }

    type ListenStream = Pin<
        Box<
            dyn tokio_stream::Stream<
                    Item = tonic::Result<teleproxy_pb::ListenResponse, tonic::Status>,
                > + Send,
        >,
    >;
    async fn listen(
        &self,
        request: tonic::Request<tonic::Streaming<teleproxy_pb::ListenRequest>>,
    ) -> tonic::Result<tonic::Response<Self::ListenStream>> {
        let mut in_stream = request.into_inner();

        let id = match in_stream.next().await {
            Some(result) => match result {
                Ok(request) => request.id,
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
        self.forward_handler.register_handler(&id, tx);

        let (stream_tx, steram_rx) = tokio::sync::mpsc::channel(128);
        let out_stream = ReceiverStream::new(steram_rx);

        tokio::spawn(async move {
            while let Some((request, response_tx)) = rx.recv().await {
                let header = request
                    .header
                    .iter()
                    .map(|header| {
                        (
                            header.0.clone(),
                            header.1.iter().fold(
                                teleproxy_pb::HeaderValues { values: Vec::new() },
                                |acc, value| {
                                    let values = [acc.values, vec![value.to_string()]].concat();
                                    teleproxy_pb::HeaderValues { values }
                                },
                            ),
                        )
                    })
                    .collect();

                let response_result = stream_tx
                    .send(Ok(teleproxy_pb::ListenResponse {
                        method: request.method,
                        url: request.url,
                        header,
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
                        response = Some(dto::Response::from_pb(request))
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
        request: tonic::Request<teleproxy_pb::DeregisterRequest>,
    ) -> tonic::Result<tonic::Response<teleproxy_pb::DeregisterResponse>> {
        let request = request.into_inner();
        log::trace!("deregister requested with payload {:?}", request);

        self.forward_config_store.remove_by_id(&request.id);

        Ok(tonic::Response::new(teleproxy_pb::DeregisterResponse {}))
    }

    async fn dump(
        &self,
        request: tonic::Request<teleproxy_pb::DumpRequest>,
    ) -> Result<tonic::Response<teleproxy_pb::DumpResponse>, tonic::Status> {
        let request = request.into_inner();
        log::trace!("dump requested with payload {:?}", request);

        let forward_config_store = Arc::clone(&self.forward_config_store);
        let config_dump = serde_yml::to_string(&forward_config_store.list());

        match config_dump {
            Ok(config_dump) => Ok(tonic::Response::new(teleproxy_pb::DumpResponse {
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
        request: tonic::Request<teleproxy_pb::FlushRequest>,
    ) -> tonic::Result<tonic::Response<teleproxy_pb::FlushResponse>> {
        let request = request.into_inner();
        log::trace!("flush requested with payload {:?}", request);

        unimplemented!()
    }
}
