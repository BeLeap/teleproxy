use crate::{dto, proto};
use http::StatusCode;

pub const INTERNAL_ERROR_RESPONSE: HttpResponse = HttpResponse {
    status_code: StatusCode::INTERNAL_SERVER_ERROR,
    headers: vec![],
    body: vec![],
};

#[derive(Debug)]
pub struct HttpResponse {
    pub status_code: StatusCode,
    pub headers: Vec<dto::header::Header>,
    pub body: Vec<u8>,
}

#[derive(Debug)]
pub enum ResponseConversionError {
    WrongStatusCode,
    WrongBody,
}

impl TryFrom<proto::teleproxy::ListenRequest> for HttpResponse {
    type Error = ResponseConversionError;

    fn try_from(value: proto::teleproxy::ListenRequest) -> Result<Self, Self::Error> {
        let downcast_status_code = match u16::try_from(value.status_code) {
            Ok(v) => v,
            Err(e) => {
                tracing::error!(err = format!("{:#?}", e), "failed to convert status code");
                return Err(ResponseConversionError::WrongStatusCode);
            }
        };
        let status_code = match StatusCode::from_u16(downcast_status_code) {
            Ok(v) => v,
            Err(e) => {
                tracing::error!(
                    err = format!("{:#?}", e),
                    "failed to convret status code from number"
                );
                return Err(ResponseConversionError::WrongStatusCode);
            }
        };
        tracing::debug!("built status code: {:?}", status_code);

        let headers = value
            .headers
            .iter()
            .map(|header| dto::header::Header {
                key: header.0.to_string(),
                value: header.1.to_string(),
            })
            .collect();
        tracing::debug!("built headers: {:?}", headers);

        let body = value.body;

        Ok(Self {
            status_code,
            headers,
            body,
        })
    }
}

impl HttpResponse {
    pub async fn from_reqwest(
        value: reqwest::Response,
    ) -> Result<HttpResponse, ResponseConversionError> {
        let status_code = value.status();

        let headers: Vec<dto::header::Header> = value
            .headers()
            .iter()
            .filter_map(|header| header.try_into().ok())
            .collect();

        let body = value.bytes().await;
        match body {
            Ok(body) => {
                let body = body.to_vec();

                Ok(HttpResponse {
                    status_code,
                    headers,
                    body,
                })
            }
            Err(err) => {
                tracing::error!(err = format!("{:#?}", err), "failed to get body");
                Err(ResponseConversionError::WrongBody)
            }
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
