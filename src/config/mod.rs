pub mod server;

#[derive(Debug, Clone)]
pub enum Error {
    ReadFile,
    Deserialize,
}
