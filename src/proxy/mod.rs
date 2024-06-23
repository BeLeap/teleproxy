use std::{collections::HashMap, net::IpAddr, sync::Arc};

use async_trait::async_trait;
use pingora::prelude::*;

use crate::{
    dto,
    forwardconfig::{header::Header, store::ForwardConfigStore},
    forwardhandler::ForwardHandler,
};

pub struct Target {
    pub ip: IpAddr,
    pub port: u16,
}

pub struct TeleproxyService {
    pub forward_config_store: Arc<ForwardConfigStore>,
    pub forward_handler: Arc<ForwardHandler>,

    pub target: Target,
}

#[async_trait]
impl ProxyHttp for TeleproxyService {
    type CTX = ();
    fn new_ctx(&self) -> Self::CTX {}

    async fn upstream_peer(
        &self,
        _session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>> {
        let peer = Box::new(HttpPeer::new(
            (self.target.ip, self.target.port),
            false,
            "".to_string(),
        ));
        Ok(peer)
    }

    async fn request_filter(&self, session: &mut Session, _ctx: &mut Self::CTX) -> Result<bool> {
        let id = session.req_header().headers.iter().fold(None, |_, header| {
            return self
                .forward_config_store
                .find_by_header(Header::from_pair(header));
        });

        match id {
            Some(id) => {
                let request = dto::Request {
                    method: "GET".to_string(),
                    url: "https://example.com".to_string(),
                    header: HashMap::new(),
                    body: Vec::new(),
                };

                let request_sender = self.forward_handler.get_sender(&id);

                let (response_tx, response_rx) = tokio::sync::oneshot::channel();
                request_sender.send((request, response_tx));

                match response_rx.await {
                    Ok(_response) => Ok(true),
                    Err(_) => Ok(false),
                }
            }
            None => Ok(false),
        }
    }
}
