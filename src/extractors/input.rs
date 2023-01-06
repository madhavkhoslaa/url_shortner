use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct UrlDetails {
    pub url: String,
    pub hash_type: hashType,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum hashType {
    base62_hash,
    collision_hash,
}
