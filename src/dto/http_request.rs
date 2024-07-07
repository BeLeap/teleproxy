use http::HeaderMap;

use crate::{client::http_client::HttpClient, dto, proto};

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

impl From<proto::teleproxy::ListenResponse> for HttpRequest {
    fn from(value: proto::teleproxy::ListenResponse) -> Self {
        let headers = value.headers.iter().map(|header| header.into()).collect();

        Self {
            method: value.method,
            uri: value.url,
            headers,
            body: value.body,
        }
    }
}

impl HttpRequest {
    pub fn into_reqwest(self, http_client: HttpClient) -> reqwest::RequestBuilder {
        let method: http::Method = match self.method.parse() {
            Ok(v) => v,
            Err(err) => {
                panic!("Received unsupported method: {}", err)
            }
        };
        let client = http_client.into_reqwest(method, self.uri);

        let headers: HeaderMap = self
            .headers
            .iter()
            .map(|header| header.into())
            .collect();
        let client = client.headers(headers);

        client.body(self.body)
    }
}
