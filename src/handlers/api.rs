use actix_web::{web, HttpResponse, Responder};
use url_shortner::core::collision_hash;
use url_shortner::extractors::input::{self};
use url_shortner::core::base62_hash;
use url_shortner::extractors::input::hashType::base62_hash as enum_base62_hash;
use url_shortner::extractors::input::hashType::collision_hash as enum_collision_hash;
use url_shortner::traits::hash::Hasher;

pub async fn shorten_url(url: web::Json<input::UrlDetails>) -> impl Responder {
    let this_ = url.clone();
    let mut result: String = String::from("Incorrect Option");
    if url.hash_type == enum_base62_hash {
        let base62_hash = base62_hash::base62_hash::new(url.url.clone());
        result = base62_hash.hash();
    }
    if url.hash_type == enum_collision_hash {
        let collision_hash = collision_hash::collision_hash::new(url.url.clone());
        result = collision_hash.hash();
    }
    HttpResponse::Ok().body(result)
}

pub async fn get_long_url() -> impl Responder {
    HttpResponse::Ok().body("Returns a long URL ")
}
