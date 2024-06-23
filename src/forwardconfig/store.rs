use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::dto::header::Header;

pub struct ForwardConfigStore {
    configs: Arc<Mutex<HashMap<Header, String>>>,
}

impl ForwardConfigStore {
    pub fn new() -> Self {
        let configs = Arc::new(Mutex::new(HashMap::new()));
        ForwardConfigStore { configs }
    }

    pub fn insert(&self, header: Header, id: &String) {
        let mut configs = self.configs.lock().unwrap();
        configs.insert(header, id.to_string());
    }

    pub fn find_by_header(&self, header: Header) -> Option<String> {
        let configs = self.configs.lock().unwrap();

        configs.get(&header).cloned()
    }

    pub fn remove_by_id(&self, id: &String) {
        let configs = self.configs.lock().unwrap();

        let matching_configs = configs.iter().find(|entry| entry.1 == id);

        let mut configs = self.configs.lock().unwrap();

        if let Some(matching_configs) = matching_configs {
            configs.remove(matching_configs.0);
        }
    }

    pub fn list(&self) -> HashMap<Header, String> {
        let configs = self.configs.lock().unwrap();

        configs.clone()
    }
}
