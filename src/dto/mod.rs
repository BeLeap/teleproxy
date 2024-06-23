use std::collections::HashMap;

use crate::server::teleproxy_pb;

pub struct Request {
    pub method: String,
    pub url: String,
    pub header: HashMap<String, Vec<String>>,
    pub body: Vec<u8>,
}
pub struct Response {}

impl Response {
    pub fn from_pb(_pb: teleproxy_pb::ListenRequest) -> Self {
        Self {}
    }
}
