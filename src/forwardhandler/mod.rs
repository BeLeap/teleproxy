use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::dto;

type Sender = tokio::sync::mpsc::Sender<(dto::Request, tokio::sync::oneshot::Sender<dto::Response>)>;
pub struct ForwardHandler {
    handlers: Arc<Mutex<HashMap<String, Sender>>>,
}

impl ForwardHandler {
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register_handler(
        &self,
        id: &String,
        tx: Sender,
    ) {
        let mut handlers = self.handlers.lock().unwrap();
        handlers.insert(id.to_string(), tx);
    }
}
