pub mod http_client;

use std::collections::HashMap;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};

use crate::{
    client::http_client::HttpClient,
    dto::{self, http_response::HttpResponse},
    proto,
};

type Client = proto::teleproxy::teleproxy_client::TeleproxyClient<tonic::transport::Channel>;

pub type ClientResult<T> = Result<T, ClientError>;
#[derive(Debug)]
pub enum ClientError {
    Register,
    Deregister,
}

pub async fn register(
    client: &mut Client,
    api_key: String,
    header_key: String,
    header_value: String,
) -> ClientResult<String> {
    match client
        .register(tonic::Request::new(proto::teleproxy::RegisterRequest {
            api_key,
            header_key,
            header_value,
        }))
        .await
    {
        Ok(response) => {
            let response = response.into_inner();
            Ok(response.id)
        }
        Err(err) => {
            log::error!("failed to register: {:?}", err);
            Err(ClientError::Register)
        }
    }
}

pub async fn deregister(client: &mut Client, api_key: String, id: String) -> ClientResult<()> {
    match client
        .deregister(tonic::Request::new(proto::teleproxy::DeregisterRequest {
            api_key,
            id,
        }))
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => {
            log::error!("failed to deregister with err: {:?}", err);
            Err(ClientError::Deregister)
        }
    }
}

pub async fn listen(
    client: &mut Client,
    api_key: &String,
    id: &String,
    target: &str,
) -> ClientResult<()> {
    let (stream_tx, stream_rx) = tokio::sync::mpsc::channel(128);
    let in_stream = ReceiverStream::new(stream_rx);
    let _ = stream_tx
        .send(proto::teleproxy::ListenRequest {
            api_key: api_key.to_string(),
            id: id.to_string(),
            phase: 0,
            status_code: 0,
            headers: HashMap::new(),
            body: Vec::new(),
        })
        .await;

    let response = client.listen(in_stream).await.unwrap();
    let mut out_stream = response.into_inner();

    loop {
        if let Some(result) = out_stream.next().await {
            let listen_response = result.unwrap();

            match listen_response.phase.try_into().unwrap() {
                dto::phase::ListenPhase::Tunneling => {
                    let http_request: dto::http_request::HttpRequest = listen_response.into();
                    let http_client = HttpClient {
                        target: target.to_string(),
                    };
                    let client = http_request.into_reqwest(http_client);
                    let http_response = client.send().await;

                    match http_response {
                        Ok(http_response) => {
                            let http_response = HttpResponse::from_reqwest(http_response).await;
                            let listen_request =
                                http_response.into_proto("".to_string(), id.clone());

                            let _ = stream_tx.send(listen_request).await;
                        }
                        Err(err) => {
                            log::error!("Failed to send request: {}", err);
                            let _ = stream_tx
                                .send(
                                    dto::http_response::INTERNAL_ERROR_RESPONSE
                                        .into_proto(api_key.to_string(), id.to_string()),
                                )
                                .await;
                        }
                    };
                }
                phase => {
                    log::error!("Unsupported phase: {:?}", phase);
                    let _ = stream_tx
                        .send(
                            dto::http_response::INTERNAL_ERROR_RESPONSE
                                .into_proto(api_key.to_string(), id.to_string()),
                        )
                        .await;
                }
            }
        }
    }
}
