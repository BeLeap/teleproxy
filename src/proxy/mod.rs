use std::net::IpAddr;

use async_trait::async_trait;
use pingora::prelude::*;

pub struct Target {
    pub ip: IpAddr,
    pub port: u16,
}

pub struct TeleProxyService {
    pub target: Target,
}

#[async_trait]
impl ProxyHttp for TeleProxyService {
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
}
