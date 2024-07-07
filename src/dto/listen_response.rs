use crate::dto;

#[derive(Debug)]
pub struct ListenResponse {
    pub method: String,
    pub uri: String,
    pub headers: Vec<dto::header::Header>,
    pub body: Vec<u8>,
}
