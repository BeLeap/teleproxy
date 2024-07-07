use crate::{dto, proto};
use http::StatusCode;

#[derive(Debug)]
pub struct HttpResponse {
    pub status_code: StatusCode,
    pub headers: Vec<dto::header::Header>,
    pub body: Vec<u8>,
}

#[derive(Debug)]
pub enum ResponseConversionError {
    WrongStatusCode,
}

impl TryFrom<proto::teleproxy::ListenRequest> for HttpResponse {
    type Error = ResponseConversionError;

    fn try_from(value: proto::teleproxy::ListenRequest) -> Result<Self, Self::Error> {
        let downcast_status_code = match u16::try_from(value.status_code) {
            Ok(v) => v,
            Err(e) => {
                log::error!("failed to convert status code: {:?}", e);
                return Err(ResponseConversionError::WrongStatusCode);
            }
        };
        let status_code = match StatusCode::from_u16(downcast_status_code) {
            Ok(v) => v,
            Err(e) => {
                log::error!("failed to convret status code from number: {:?}", e);
                return Err(ResponseConversionError::WrongStatusCode);
            }
        };
        log::debug!("built status code: {:?}", status_code);

        let headers = value
            .headers
            .iter()
            .map(|header| dto::header::Header {
                key: header.0.to_string(),
                value: header.1.to_string(),
            })
            .collect();
        log::debug!("built headers: {:?}", headers);

        let body = value.body;

        Ok(Self {
            status_code,
            headers,
            body,
        })
    }
}

impl HttpResponse {
    pub async fn from_reqwest(value: reqwest::Response) -> HttpResponse {
        let status_code = value.status();

        let headers: Vec<dto::header::Header> = value
            .headers()
            .iter()
            .filter_map(|header| header.try_into().ok())
            .collect();

        let body = value.bytes().await;
        let body = match body {
            Ok(v) => v,
            Err(err) => {
                panic!("Failed to get body: {}", err)
            }
        };
        let body = body.to_vec();

        HttpResponse {
            status_code,
            headers,
            body,
        }
    }

    pub fn into_proto(self, api_key: String, id: String) -> proto::teleproxy::ListenRequest {
        let headers = self
            .headers
            .iter()
            .map(|header| header.clone().into())
            .collect();

        proto::teleproxy::ListenRequest {
            api_key,
            id,
            phase: dto::phase::ListenPhase::Tunneling as i32,
            status_code: self.status_code.as_u16() as i32,
            headers,
            body: self.body,
        }
    }
}
