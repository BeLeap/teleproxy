use http::{HeaderName, HeaderValue};
use serde::Serialize;
use std::hash::Hash;

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

#[derive(Debug)]
pub enum HeaderConversionError {
    InvalidHeaderValue,
}

impl TryFrom<(&HeaderName, &HeaderValue)> for Header {
    type Error = HeaderConversionError;

    fn try_from((name, value): (&HeaderName, &HeaderValue)) -> Result<Self, Self::Error> {
        let value = match value.to_str() {
            Ok(v) => v,
            Err(err) => {
                tracing::error!(
                    err = format!("{:#?}", err),
                    "failed to convert header value",
                );
                return Err(HeaderConversionError::InvalidHeaderValue);
            }
        }
        .to_string();
        Ok(Self {
            key: name.to_string(),
            value,
        })
    }
}

impl From<Header> for (String, String) {
    fn from(val: Header) -> Self {
        (val.key, val.value)
    }
}

impl From<(&String, &String)> for Header {
    fn from(value: (&String, &String)) -> Self {
        Header {
            key: value.0.clone(),
            value: value.1.clone(),
        }
    }
}

impl From<&Header> for (reqwest::header::HeaderName, reqwest::header::HeaderValue) {
    fn from(value: &Header) -> Self {
        let key = reqwest::header::HeaderName::from_bytes(value.key.as_bytes()).unwrap();
        (key, value.value.parse().unwrap())
    }
}
