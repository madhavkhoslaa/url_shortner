use actix_web::{web, App, HttpServer};
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/shorten", web::post().to(handlers::api::shorten_url))
            .route("/shorten", web::get().to(handlers::api::get_long_url))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

// fn main() {
//     let new_redis = database::redis_proxy::RedisProxy::new(String::from("redis://127.0.0.1/"));
//     let res_ = new_redis.set("Key", "Value");
//     let result = new_redis.get("Key");
//     println!("{}", result.unwrap());
// }
