use std::{net::IpAddr, sync::Arc};

use async_trait::async_trait;
use pingora::{http::ResponseHeader, prelude::*};
use prost::bytes::Bytes;

use crate::{dto, forwardconfig::store::ForwardConfigStore, forwardhandler::ForwardHandler};

pub struct TeleproxyPingoraService {
    pub forward_config_store: Arc<ForwardConfigStore>,
    pub forward_handler: Arc<ForwardHandler>,
    pub target: (IpAddr, u16),
}

#[async_trait]
impl ProxyHttp for TeleproxyPingoraService {
    type CTX = ();
    fn new_ctx(&self) -> Self::CTX {}

    async fn request_filter(
        &self,
        session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> pingora_core::Result<bool> {
        let id = session.req_header().headers.iter().fold(None, |_, header| {
            match dto::header::Header::try_from(header) {
                Ok(v) => self.forward_config_store.find_by_header(v),
                Err(err) => {
                    log::error!("Failed to convert header: {:?}", err);
                    None
                }
            }
        });

        match id {
            None => Ok(false),
            Some(id) => {
                log::info!("forwarding requsest to id: {}", id);
                let body = match session.read_request_body().await {
                    Ok(v) => match v {
                        Some(v) => v.to_vec(),
                        None => vec![],
                    },
                    Err(_e) => vec![],
                };
                let req_header = session.req_header();
                let request = dto::http_request::HttpRequest {
                    method: req_header.method.to_string(),
                    uri: req_header.uri.to_string(),
                    headers: req_header
                        .headers
                        .iter()
                        .filter_map(|header| dto::header::Header::try_from(header).ok())
                        .collect(),
                    body,
                };

                let request_sender = self.forward_handler.get_sender(&id);

                let (response_tx, response_rx) = tokio::sync::oneshot::channel();
                let _ = request_sender.send((request, response_tx)).await;

                match response_rx.await {
                    Ok(response) => {
                        log::trace!("Received response for forwarded request: {:?}", response);
                        let mut response_header =
                            ResponseHeader::build(response.status_code.as_u16(), None).unwrap();
                        for header in response.headers {
                            let _ = response_header.append_header(header.key, header.value);
                        }

                        if let Err(err) = session
                            .write_response_header(Box::new(response_header))
                            .await
                        {
                            log::error!("write_response_header error: {:?}", err);
                            return Err(pingora_core::Error::new(ErrorType::InternalError));
                        };

                        if let Err(err) = session
                            .write_response_body(Bytes::from(response.body))
                            .await
                        {
                            log::error!("write_response_body error: {:?}", err);
                            return Err(pingora_core::Error::new(ErrorType::InternalError));
                        };

                        session.set_keepalive(None);

                        Ok(true)
                    }
                    Err(err) => {
                        log::error!("response_rx error: {:?}", err);

                        Err(pingora_core::Error::new(ErrorType::InternalError))
                    }
                }
            }
        }
    }

    async fn upstream_peer(
        &self,
        _session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>> {
        let peer = Box::new(HttpPeer::new(
            (self.target.0, self.target.1),
            false,
            "".to_string(),
        ));
        Ok(peer)
    }
}
