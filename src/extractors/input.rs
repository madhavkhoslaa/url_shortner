use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct UrlDetails {
    pub url: String,
    pub hash_type: HashType,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum HashType {
    Base62Hash,
    CollisionHash,
}
