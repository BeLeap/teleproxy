use std::{
    hash::Hash,
    sync::{Arc, Mutex},
};

use crate::dto::{Request, Response};

use super::header::Header;

pub type Handler = Box<dyn Fn(Request) -> Response + Send + 'static>;

pub struct ForwardConfig {
    pub header: Header,
    pub handler: Option<Arc<Mutex<Handler>>>,
}

impl Hash for ForwardConfig {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.header.hash(state);
    }
}

impl PartialEq for ForwardConfig {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header
    }
}
impl Eq for ForwardConfig {}
