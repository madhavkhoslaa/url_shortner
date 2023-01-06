use crate::traits;

pub struct collision_hash {
    url: String,
    hashed_url: Option<String>,
}

impl collision_hash {
    pub fn new(url: String) -> collision_hash {
        collision_hash {
            url,
            hashed_url: None,
        }
    }
    fn get_idx(self) -> usize {
        todo!();
    }
}

impl traits::hash::Hasher for collision_hash {
    fn hash(&self) -> String {
        String::from("Collision Hash")
    }
}
