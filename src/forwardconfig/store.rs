use std::{collections::HashSet, sync::{Arc, Mutex}};

use super::{config::{self, ForwardConfig}, header::Header, };

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
        handler: config::Handler,
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
    ) -> Option<Arc<Mutex<config::Handler>>> {
        let configs = self.configs.lock().unwrap();

        match configs.get(&ForwardConfig {
            header,
            handler: None,
        }) {
            Some(config) => config.handler.clone(),
            None => None,
        }
    }
}

