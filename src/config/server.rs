use std::fs;

use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Server {
    pub target_ip: String,
    pub target_port: u16,
    pub port: u16,
    pub server_port: u16,
    pub api_key: String,
}

impl Server {
    pub fn read(path: &String) -> Result<Server, super::Error> {
        match fs::read_to_string(path) {
            Ok(content) => match serde_yml::from_str::<Server>(&content) {
                Ok(config) => Ok(config),
                Err(err) => {
                    tracing::warn!(
                        err = format!("{:#?}", err),
                        "failed to desrealize file at path {}",
                        path,
                    );
                    Err(super::Error::Deserialize)
                }
            },
            Err(err) => {
                tracing::warn!(
                    err = format!("{:#?}", err),
                    "failed to read config file at path {}",
                    path
                );
                Err(super::Error::ReadFile)
            }
        }
    }
}
