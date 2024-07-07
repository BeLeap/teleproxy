pub mod header;
pub mod phase;
pub mod listen_response;
pub mod listen_request;

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub uri: String,
    pub headers: Vec<header::Header>,
    pub body: Vec<u8>,
}
