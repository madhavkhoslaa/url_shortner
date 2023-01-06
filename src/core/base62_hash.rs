use super::database_core::DatabaseCore;
use crate::traits;

pub struct Base62Hash {
    url: String,
    hashed_url: Option<String>,
    db: DatabaseCore,
}

impl Base62Hash {
    pub fn new(url: String, db: DatabaseCore) -> Base62Hash {
        Base62Hash {
            url,
            hashed_url: None,
            db,
        }
    }
}

impl traits::hash::Hasher for Base62Hash {
    // All the urls are supposed to be the shortest possible
    // Hence we are using indexes from the DB and not random number generation
    // char(n) => 64^n size url
    fn hash(&mut self) -> String {
        // 0. Check if string is already
        if self.db.find_collision(String::from(&self.url)) {
            return self.db.client.get(&self.url).unwrap();
        }
        // 1. Get index for the hash
        let idx: usize = self.db.get_count();
        // 2. Generate hash
        let hash = base62::encode(idx as u128);
        // 3. Set hash
        self.db.client.set(&self.url, &hash).unwrap();
        // 4. Increase index count
        self.db.update_count();
        self.hashed_url = Some(String::from(&hash));
        return String::from(&hash);
    }
}
