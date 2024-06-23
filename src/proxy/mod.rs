use std::{
    net::IpAddr,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use pingora::prelude::*;

use crate::forwardconfig::{self, header::Header, store::ForwardConfigStore};

pub struct Target {
    pub ip: IpAddr,
    pub port: u16,
}

pub struct TeleproxyService {
    forward_config_store: Arc<ForwardConfigStore>,

    target: Target,
}

impl TeleproxyService {
    pub fn new(forward_config_store: Arc<ForwardConfigStore>, target: Target) -> Self {
        Self {
            forward_config_store,
            target,
        }
    }
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
        todo!()
        // let handler = session
        //     .req_header()
        //     .headers
        //     .iter()
        //     .map(|header| {
        //         self.forward_config_store
        //             .find_by_header(Header::from_pair(header))
        //     })
        //     .collect::<Vec<Option<Arc<Mutex<forwardconfig::config::Handler>>>>>()
        //     .last();
        // Ok(false)
    }
}
