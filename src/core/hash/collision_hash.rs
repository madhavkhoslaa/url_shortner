use md5::Digest;

use crate::{core::database_core::DatabaseCore, traits};

pub struct collision_hash {
    url: String,
    hashed_url: Option<String>,
    db: DatabaseCore,
}

impl collision_hash {
    pub fn new(url: String, db: DatabaseCore) -> collision_hash {
        collision_hash {
            url,
            hashed_url: None,
            db,
        }
    }
}

impl traits::hash::Hasher for collision_hash {
    fn hash(&mut self) -> String {
        // 0. Check if the string hsa already been hashed
        if self.db.find_collision(String::from(&self.url)) {
            return self.db.client.get(&self.url).unwrap();
        }
        // 1. If not generate the hash
        let mut hash: String = format!("{:x}", md5::Md5::digest(&self.url));
        // 2. Check if hash collides
        while self.db.find_collision(hash.clone()) {
            // 3. if hash collides append a string to the url string and hash and repeat until you find a uncolided hash
            hash.push_str(" ")
        }
        // 4. Set the hash
        self.hashed_url = Some(hash.clone());
        let _res = self.db.client.set(&hash, &self.url);
        hash
    }
}
