use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use super::{
    config::ForwardConfig,
    header::Header,
};

pub struct ForwardConfigStore {
    configs: Arc<Mutex<HashSet<ForwardConfig>>>,
}

impl ForwardConfigStore {
    pub fn new() -> Self {
        let configs = Arc::new(Mutex::new(HashSet::new()));
        ForwardConfigStore { configs }
    }

    pub fn insert(&self, header: Header, id: &String) {
        let config = ForwardConfig {
            header,
            id: id.to_string(),
        };

        let mut configs = self.configs.lock().unwrap();
        configs.insert(config);
    }

    pub fn find_by_header(&self, header: Header) -> Option<String> {
        let configs = self.configs.lock().unwrap();

        configs
            .get(&ForwardConfig {
                header,
                id: "".to_string(),
            })
            .map(|matching| matching.id.clone())
    }

    pub fn list(&self) -> Vec<Header> {
        let configs = &self.configs.lock().unwrap();

        configs.iter().map(|config| config.header.clone()).collect()
    }
}
