use actix_web::{web, HttpResponse, Responder};
use url_shortner::core::base62_hash;
use url_shortner::core::collision_hash;
use url_shortner::core::database_core;
use url_shortner::extractors::input::HashType::Base62Hash;
use url_shortner::extractors::input::HashType::CollisionHash;
use url_shortner::extractors::input::{self};
use url_shortner::traits::hash::Hasher;

pub async fn shorten_url(url: web::Json<input::UrlDetails>) -> impl Responder {
    let mut result: String = String::from("Incorrect Option");
    if url.hash_type == Base62Hash {
        let mut base62_hash =
            base62_hash::Base62Hash::new(url.url.clone(), database_core::DatabaseCore::new());
        result = base62_hash.hash();
    }
    if url.hash_type == CollisionHash {
        let mut collision_hash = collision_hash::collision_hash::new(
            url.url.clone(),
            database_core::DatabaseCore::new(),
        );
        result = collision_hash.hash();
    }
    HttpResponse::Ok().body(result)
}

pub async fn get_long_url(url: web::Json<input::UrlDetails>) -> impl Responder {
    // Remove DB logic from here
    // Take url data as string parameter
    // Return redirection logic
    // Save X-Agent details to a DB for analytics
    let db = database_core::DatabaseCore::new();
    let mut result = String::from("Not Found");
    if db.client.has(&url.url) {
        result = db.client.get(&url.url).unwrap();
    }
    HttpResponse::Ok().body(result)
}
