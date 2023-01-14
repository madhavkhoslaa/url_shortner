use crate::core::database_core::DatabaseCore;

use super::{base62_hash::Base62Hash, collision_hash::collision_hash};

pub struct Factory {}

impl Factory {
    pub fn base62(url: String, db: DatabaseCore) -> Base62Hash {
        Base62Hash::new(url, db)
    }
    pub fn collision_hash(url: String, db: DatabaseCore) -> collision_hash {
        collision_hash::new(url, db)
    }
}
