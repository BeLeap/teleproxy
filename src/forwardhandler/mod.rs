use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::dto;

type Sender = tokio::sync::mpsc::Sender<(
    dto::http_request::HttpRequest,
    tokio::sync::oneshot::Sender<dto::http_response::HttpResponse>,
)>;
pub struct ForwardHandler {
    handlers: Arc<Mutex<HashMap<String, Sender>>>,
}

impl ForwardHandler {
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register_sender(&self, id: &String, sender: Sender) {
        // TODO: remove unwrap
        let mut handlers = self.handlers.lock().unwrap();
        handlers.insert(id.to_string(), sender);
    }

    pub fn get_sender(&self, id: &String) -> Sender {
        // TODO: remove unwrap
        let handlers = self.handlers.lock().unwrap();
        handlers.get(id).unwrap().clone()
    }
}
