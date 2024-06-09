use std::{collections::HashSet, sync::{Arc, Mutex}};

use crate::dto::{Request, Response};

use super::{config::ForwardConfig, header::Header, };

pub struct ForwardConfigStore {
    configs: Arc<Mutex<HashSet<ForwardConfig>>>,
}

impl ForwardConfigStore {
    pub fn new() -> Self {
        let configs = Arc::new(Mutex::new(HashSet::new()));
        ForwardConfigStore { configs }
    }

    pub fn insert(
        &self,
        header: Header,
        handler: Box<dyn Fn(Request) -> Response + Send + 'static>,
    ) {
        let config = ForwardConfig {
            header,
            handler: Some(Arc::new(Mutex::new(handler))),
        };

        let mut configs = self.configs.lock().unwrap();
        configs.insert(config);
    }

    pub fn find_by_header(
        &self,
        header: Header,
    ) -> Option<Arc<Mutex<Box<dyn Fn(Request) -> Response + Send>>>> {
        let configs = self.configs.lock().unwrap();

        match configs.get(&ForwardConfig {
            header,
            handler: None,
        }) {
            Some(config) => match &config.handler {
                Some(handler) => Some(handler.clone()),
                None => None,
            },
            None => None,
        }
    }
}

