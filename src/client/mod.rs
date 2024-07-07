use std::collections::HashMap;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};

use crate::{dto::{self}, proto};

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
                    let client = reqwest::Client::new();

                    let method: http::Method = match listen_response.method.parse() {
                        Ok(v) => v,
                        Err(err) => {
                            panic!("Received unsupported method: {}", err)
                        }
                    };
                    let url = format!("{}{}", target, listen_response.url);
                    log::debug!("Sending request to {:?}", url);
                    let url = url.parse::<reqwest::Url>().unwrap();
                    let client = client.request(method, url);

                   let mut headers = reqwest::header::HeaderMap::new();
                    for header in listen_response.headers {
                        let key = reqwest::header::HeaderName::from_bytes(header.0.as_bytes())
                            .unwrap();
                        headers.insert(key, header.1.parse().unwrap());
                    }
                    let client = client.headers(headers);

                    let client = client.body(listen_response.body);
                    let http_response = client.send().await;

                    let mut http_response = match http_response {
                        Ok(v) => v,
                        Err(err) => {
                            panic!("Failed to send request: {}", err)
                        },
                    };

                    let status_code = http_response.status().as_u16() as i32;

                    let headers = http_response.headers_mut();
                    let headers = headers
                        .iter_mut()
                        .map(|header| {
                            (header.0.to_string(), header.1.to_str().unwrap().to_string())
                        })
                        .collect();

                    let body = http_response.bytes().await;
                    let body = match body {
                        Ok(v) => v,
                        Err(err) => {
                            panic!("Failed to get body: {}", err)
                        },
                    };
                    let body = body.to_vec();

                    let listen_request = proto::teleproxy::ListenRequest {
                        api_key: "".to_string(),
                        id: id.to_string(),
                        phase: dto::phase::ListenPhase::Tunneling as i32,
                        status_code,
                        headers,
                        body,
                    };
                    println!("Sending {:?}", listen_request);

                    let _ = stream_tx.send(listen_request).await;
                }
                phase => {
                    panic!("Unsupported phase: {:?}", phase)
                }
            }
        }
    }
}
