pub mod header;
pub mod phase;

use http::StatusCode;
use crate::proto;

use self::header::Header;

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub uri: String,
    pub headers: Vec<header::Header>,
    pub body: Vec<u8>,
}
#[derive(Debug)]
pub struct Response {
    pub status_code: StatusCode,
    pub headers: Vec<header::Header>,
    pub body: Vec<u8>,
}

impl Response {
    pub fn from_pb(pb: proto::teleproxy::ListenRequest) -> Self {
        let downcast_status_code = match u16::try_from(pb.status_code) {
            Ok(v) => v,
            Err(e) => {
                log::error!("failed to convert status code: {:?}", e);
                panic!()
            }
        };
        let status_code = match StatusCode::from_u16(downcast_status_code) {
            Ok(v) => v,
            Err(e) => {
                log::error!("failed to convret status code from number: {:?}", e);
                panic!()
            }
        };
        log::debug!("built status code: {:?}", status_code);

        let headers = pb
            .headers
            .iter()
            .map(|header| Header {
                key: header.0.to_string(),
                value: header.1.to_string(),
            })
            .collect();
        log::debug!("built headers: {:?}", headers);

        let body = pb.body;

        Self {
            status_code,
            headers,
            body,
        }
    }
}
