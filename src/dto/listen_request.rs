use crate::{proto, dto};
use http::StatusCode;

#[derive(Debug)]
pub struct ListenRequest {
    pub status_code: StatusCode,
    pub headers: Vec<dto::header::Header>,
    pub body: Vec<u8>,
}

#[derive(Debug)]
pub enum ResponseConversionError {
    WrongStatusCode,
}

impl TryFrom<proto::teleproxy::ListenRequest> for ListenRequest {
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
