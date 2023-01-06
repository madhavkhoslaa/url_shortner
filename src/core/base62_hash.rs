use crate::traits;

pub struct base62_hash {
    url: String,
    hashed_url: Option<String>,
}

impl base62_hash {
    pub fn new(url: String) -> base62_hash {
        base62_hash {
            url,
            hashed_url: None,
        }
    }
    fn get_idx(self) -> usize {
        todo!();
    }
}

impl traits::hash::Hasher for base62_hash {
    fn hash(&self) -> String {
        String::from("base62_hash Hash")
    }
}
