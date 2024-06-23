use http::{HeaderName, HeaderValue};
use serde::Serialize;

#[derive(PartialEq, Eq, Hash, Clone, Serialize)]
pub struct Header {
    pub key: String,
    pub value: String,
}

impl Header {
    pub fn from_pair((name, value): (&HeaderName, &HeaderValue)) -> Self {
        Self {
            key: name.to_string(),
            value: value.to_str().unwrap().to_string(),
        }
    }
}
