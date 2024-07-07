use std::fs;

use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Server {
    pub target_ip: String,
    pub target_port: u16,
    pub port: u16,
    pub server_port: u16,
}

impl Server {
    pub fn read(path: &String) -> Result<Server, super::Error> {
        match fs::read_to_string(path) {
            Ok(content) => match serde_yml::from_str::<Server>(&content) {
                Ok(config) => Ok(config),
                Err(err) => {
                    log::warn!("Failed to desrealize file at path {}: {}", path, err);
                    Err(super::Error::Deserialize)
                }
            },
            Err(err) => {
                log::warn!("Failed to read config file at path {}: {}", path, err);
                Err(super::Error::ReadFile)
            }
        }
    }
}
