use crate::{dto, proto};

#[derive(Debug)]
pub struct HttpRequest {
    pub method: String,
    pub uri: String,
    pub headers: Vec<dto::header::Header>,
    pub body: Vec<u8>,
}

impl From<HttpRequest> for proto::teleproxy::ListenResponse {
    fn from(val: HttpRequest) -> Self {
        let headers = val
            .headers
            .iter()
            .map(|header| header.clone().into())
            .collect();

        proto::teleproxy::ListenResponse {
            phase: dto::phase::ListenPhase::Tunneling as i32,
            method: val.method,
            url: val.uri,
            headers,
            body: val.body,
        }
    }
}
