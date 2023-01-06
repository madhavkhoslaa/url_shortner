use actix_web::{web, HttpResponse, Responder};
use url_shortner::core::base62_hash;
use url_shortner::core::collision_hash;
use url_shortner::extractors::input::HashType::Base62Hash ;
use url_shortner::extractors::input::HashType::CollisionHash ;
use url_shortner::extractors::input::{self};
use url_shortner::traits::hash::Hasher;

pub async fn shorten_url(url: web::Json<input::UrlDetails>) -> impl Responder {
    let mut result: String = String::from("Incorrect Option");
    if url.hash_type == Base62Hash {
        let base62_hash = base62_hash::base62_hash::new(url.url.clone());
        result = base62_hash.hash();
    }
    if url.hash_type == CollisionHash {
        let collision_hash = collision_hash::collision_hash::new(url.url.clone());
        result = collision_hash.hash();
    }
    HttpResponse::Ok().body(result)
}

pub async fn get_long_url() -> impl Responder {
    HttpResponse::Ok().body("Returns a long URL ")
}
