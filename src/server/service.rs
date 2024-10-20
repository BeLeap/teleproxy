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
        tracing::debug!(request = format!("{:#?}", request), "health requested");

        let response = proto::teleproxy::EchoResponse {};
        Ok(tonic::Response::new(response))
    }

    async fn register(
        &self,
        request: tonic::Request<proto::teleproxy::RegisterRequest>,
    ) -> tonic::Result<tonic::Response<proto::teleproxy::RegisterResponse>> {
        let request = request.into_inner();
        tracing::debug!(request = format!("{:#?}", request), "register requested");

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
                    tracing::error!(err = format!("{:#?}", e), "unknown error");
                    return Err(tonic::Status::internal(format!("{:?}", e)));
                }
            },
            None => {
                tracing::error!("failed to get first request");
                return Err(tonic::Status::internal("Failed to get first request"));
            }
        };
        let (tx, mut rx) = tokio::sync::mpsc::channel::<(
            dto::http_request::HttpRequest,
            tokio::sync::oneshot::Sender<dto::http_response::HttpResponse>,
        )>(128);
        tracing::info!(id, "register sender to handler");
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
                                        tracing::error!(
                                            id,
                                            err = format!("{:#?}", err),
                                            "failed to convert to dto"
                                        );
                                        break dto::http_response::INTERNAL_ERROR_RESPONSE;
                                    }
                                }
                            }
                        };
                        response_tx.send(response)
                    }
                    Err(err) => {
                        tracing::error!(id, err = format!("{:#?}", err), "failed to send request");
                        response_tx.send(dto::http_response::INTERNAL_ERROR_RESPONSE)
                    }
                };

                match pass_response_result {
                    Ok(_) => {}
                    Err(err) => {
                        tracing::error!(id, err = format!("{:#?}", err), "failed to pass response");
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
        tracing::debug!(request = format!("{:#?}", request), "deregister requested");

        if request.api_key != self.api_key {
            return Err(tonic::Status::unauthenticated("Invalid api key"));
        }

        tracing::info!(id = request.id, "remove from config");
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
        tracing::debug!(request = format!("{:#?}", request), "dump requested");

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
                tracing::error!(err = format!("{:#?}", err), "failed to dump config");
                Err(Status::internal("failed to dump config"))
            }
        }
    }

    async fn flush(
        &self,
        request: tonic::Request<proto::teleproxy::FlushRequest>,
    ) -> tonic::Result<tonic::Response<proto::teleproxy::FlushResponse>> {
        let request = request.into_inner();
        tracing::debug!(request = format!("{:#?}", request), "flush requested");

        if request.api_key != self.api_key {
            return Err(tonic::Status::unauthenticated("Invalid api key"));
        }

        unimplemented!()
    }
}
