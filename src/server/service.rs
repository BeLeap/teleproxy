use crate::{
    dto::{self, header::Header, phase::ListenPhase},
    forwardconfig::store::ForwardConfigStore,
    forwardhandler::ForwardHandler,
    proto,
};
use std::{pin::Pin, sync::Arc};
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::Status;

pub struct TeleproxyImpl {
    pub api_key: String,
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

        if request.api_key != self.api_key {
            return Err(tonic::Status::unauthenticated("Invalid api key"));
        }

        let id = ulid::Ulid::new().to_string();

        self.forward_config_store.insert(
            Header {
                key: request.header_key,
                value: request.header_value,
            },
            &id,
        );

        Ok(tonic::Response::new(proto::teleproxy::RegisterResponse {
            id,
        }))
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
                    if request.api_key != self.api_key {
                        return Err(tonic::Status::unauthenticated("Invalid api key"));
                    }

                    if request.phase == ListenPhase::Init as i32 {
                        request.id
                    } else {
                        return Err(tonic::Status::invalid_argument(
                            "First request for listen must be INIT",
                        ));
                    }
                }
                Err(e) => {
                    log::error!("{:?}", e);
                    return Err(tonic::Status::internal(format!("{:?}", e)));
                }
            },
            None => {
                log::error!("Failed to get first request");
                return Err(tonic::Status::internal("Failed to get first request"));
            }
        };
        let (tx, mut rx) = tokio::sync::mpsc::channel::<(
            dto::http_request::HttpRequest,
            tokio::sync::oneshot::Sender<dto::http_response::HttpResponse>,
        )>(128);
        self.forward_handler.register_sender(&id, tx);

        let (stream_tx, steram_rx) = tokio::sync::mpsc::channel(128);
        let out_stream = ReceiverStream::new(steram_rx);

        tokio::spawn(async move {
            while let Some((request, response_tx)) = rx.recv().await {
                let pass_request_result = stream_tx.send(Ok(request.into())).await;
                let pass_response_result = match pass_request_result {
                    Ok(_) => {
                        let response = loop {
                            if let Some(Ok(request)) = in_stream.next().await {
                                match dto::http_response::HttpResponse::try_from(request) {
                                    Ok(response) => {
                                        break response;
                                    }
                                    Err(err) => {
                                        log::error!("Failed to convert to dto: {:?}", err);
                                        break dto::http_response::INTERNAL_ERROR_RESPONSE;
                                    }
                                }
                            }
                        };
                        response_tx.send(response)
                    }
                    Err(err) => {
                        log::error!("Failed to send request: {:?}", err);
                        response_tx.send(dto::http_response::INTERNAL_ERROR_RESPONSE)
                    }
                };

                match pass_response_result {
                    Ok(_) => {}
                    Err(err) => {
                        log::error!("Failed to pass response: {:?}", err);
                    }
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

        if request.api_key != self.api_key {
            return Err(tonic::Status::unauthenticated("Invalid api key"));
        }

        self.forward_config_store.remove_by_id(&request.id);

        Ok(tonic::Response::new(
            proto::teleproxy::DeregisterResponse {},
        ))
    }

    async fn dump(
        &self,
        request: tonic::Request<proto::teleproxy::DumpRequest>,
    ) -> Result<tonic::Response<proto::teleproxy::DumpResponse>, tonic::Status> {
        let request = request.into_inner();
        log::trace!("dump requested with payload {:?}", request);

        if request.api_key != self.api_key {
            return Err(tonic::Status::unauthenticated("Invalid api key"));
        }

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

        if request.api_key != self.api_key {
            return Err(tonic::Status::unauthenticated("Invalid api key"));
        }

        unimplemented!()
    }
}
