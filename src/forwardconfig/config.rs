use super::header::Header;
use std::hash::Hash;

pub struct ForwardConfig {
    pub header: Header,
    pub id: String,
}

impl Hash for ForwardConfig {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.header.hash(state);
    }
}

impl PartialEq for ForwardConfig {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header
    }
}
impl Eq for ForwardConfig {}
