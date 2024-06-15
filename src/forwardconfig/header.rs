use http::{HeaderName, HeaderValue};
use serde::Serialize;

#[derive(PartialEq, Eq, Hash, Clone, Serialize)]
pub struct Header {
    name: String,
    value: String,
}

impl Header {
    pub fn from_pair((name, value): (&HeaderName, &HeaderValue)) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_str().unwrap().to_string(),
        }
    }
}
