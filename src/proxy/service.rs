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
            self.forward_config_store
                .find_by_header(dto::header::Header::from_pair(header))
        });

        log::info!("{:?}", id);

        match id {
            Some(id) => {
                log::info!("forwarding requsest to id: {}", id);
                let body = session.read_request_body().await.unwrap().unwrap();
                let req_header = session.req_header();
                let request = dto::Request {
                    method: req_header.method.to_string(),
                    uri: req_header.uri.to_string(),
                    headers: req_header
                        .headers
                        .iter()
                        .map(dto::header::Header::from_pair)
                        .collect(),
                    body: body.to_vec(),
                };

                let request_sender = self.forward_handler.get_sender(&id);

                let (response_tx, response_rx) = tokio::sync::oneshot::channel();
                let _ = request_sender.send((request, response_tx)).await;

                match response_rx.await {
                    Ok(response) => {
                        if let Err(err) = session
                            .write_response_body(Bytes::from(response.body))
                            .await
                        {
                            log::error!("write_response_body error: {:?}", err);
                            return Err(pingora_core::Error::new(ErrorType::InternalError));
                        };

                        let response_header =
                            ResponseHeader::build(response.status_code.as_u16(), None).unwrap();
                        if let Err(err) = session
                            .write_response_header(Box::new(response_header))
                            .await
                        {
                            log::error!("write_response_header error: {:?}", err);
                            return Err(pingora_core::Error::new(ErrorType::InternalError));
                        };
                        Ok(true)
                    }
                    Err(err) => {
                        log::error!("response_rx error: {:?}", err);

                        Err(pingora_core::Error::new(ErrorType::InternalError))
                    }
                }
            }
            None => Ok(false),
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
