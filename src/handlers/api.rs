use actix_web::{web, HttpResponse, Responder};
use url_shortner::core::base62_hash;
use url_shortner::core::collision_hash;
use url_shortner::extractors::input::HashType::Base62Hash;
use url_shortner::extractors::input::HashType::CollisionHash;
use url_shortner::extractors::input::{self};
use url_shortner::state::app_state::AppState;
use url_shortner::traits::hash::Hasher;

pub async fn shorten_url(
    url: web::Json<input::UrlDetails>,
    app_data: web::Data<AppState>,
) -> impl Responder {
    let mut result: String = String::from("Incorrect Option");
    if url.hash_type == Base62Hash {
        let mut base62_hash =
            base62_hash::Base62Hash::new(url.url.clone(), app_data.database.clone());
        result = base62_hash.hash();
    }
    if url.hash_type == CollisionHash {
        let mut collision_hash =
            collision_hash::collision_hash::new(url.url.clone(), app_data.database.clone());
        result = collision_hash.hash();
    }
    HttpResponse::Ok().body(result)
}

pub async fn get_long_url(
    hash: web::Path<(String,)>,
    app_data: web::Data<AppState>,
) -> impl Responder {
    // Remove DB logic from here
    // Take url data as string parameter
    // Return redirection logic
    // Save X-Agent details to a DB for analytics
    let hash_ = &hash.clone().0;
    let db = app_data.database.clone();
    let mut result = String::from("Not Found");
    if db.client.has(hash_) {
        result = db.client.get(&hash_).unwrap();
    }
    HttpResponse::TemporaryRedirect()
        .append_header(("Location", result))
        .finish()
}
