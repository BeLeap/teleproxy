use http::{HeaderName, HeaderValue};

#[derive(PartialEq, Eq, Hash)]
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
