use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::dto;

type Sender =
    tokio::sync::mpsc::Sender<(dto::listen_response::ListenResponse, tokio::sync::oneshot::Sender<dto::listen_request::ListenRequest>)>;
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
        let mut handlers = self.handlers.lock().unwrap();
        handlers.insert(id.to_string(), sender);
    }

    pub fn get_sender(&self, id: &String) -> Sender {
        let handlers = self.handlers.lock().unwrap();
        handlers.get(id).unwrap().clone()
    }
}
