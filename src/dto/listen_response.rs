use crate::{dto, proto};

#[derive(Debug)]
pub struct HttpRequest {
    pub method: String,
    pub uri: String,
    pub headers: Vec<dto::header::Header>,
    pub body: Vec<u8>,
}

impl Into<proto::teleproxy::ListenResponse> for HttpRequest {
    fn into(self) -> proto::teleproxy::ListenResponse {
        let headers = self
            .headers
            .iter()
            .map(|header| header.clone().into())
            .collect();

        proto::teleproxy::ListenResponse {
            phase: dto::phase::ListenPhase::Tunneling as i32,
            method: self.method,
            url: self.uri,
            headers,
            body: self.body,
        }
    }
}
