use std::hash::Hash;
use http::{HeaderName, HeaderValue};
use pingora::http::ResponseHeader;
use serde::Serialize;

#[derive(Eq, Clone, Serialize, Debug)]
pub struct Header {
    pub key: String,
    pub value: String,
}

impl Hash for Header {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key.to_lowercase().hash(state);
        self.value.hash(state);
    }
}

impl PartialEq for Header {
    fn eq(&self, other: &Self) -> bool {
        self.key.to_lowercase() == other.key.to_lowercase() && self.value == other.value
    }
}

impl Header {
    pub fn from_pair((name, value): (&HeaderName, &HeaderValue)) -> Self {
        Self {
            key: name.to_string(),
            value: value.to_str().unwrap().to_string(),
        }
    }
}

pub enum HeaderConversionError {}

impl TryInto<pingora::http::ResponseHeader> for Header {
    type Error = HeaderConversionError;

    fn try_into(self) -> Result<pingora::http::ResponseHeader, Self::Error> {
        todo!()
    }
}
