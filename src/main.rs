use actix_web::{web, App, HttpServer};
mod handlers;
use md5::{Digest, Md5};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialise database core here so that it can be passed in application states
    HttpServer::new(|| {
        App::new()
            .route("/shorten", web::post().to(handlers::api::shorten_url))
            .route("/shorten", web::get().to(handlers::api::get_long_url))
    })
    .bind(("127.0.0.1", 8001))?
    .run()
    .await
}

// fn main() {
//     // let new_redis = url_shortner::database::redis_proxy::RedisProxy::new(String::from("redis://127.0.0.1/"));
//     // let res_ = new_redis.set("Key", "Value");
//     // let result = new_redis.get("Key");
//     // println!("{:?}", new_redis.has("Key"));
//     let mut hasher = Md5::new();

//     // process input message
//     // hasher.update(b"hello world");
//     // acquire hash digest in the form of GenericArray,
//     // which in this case is equivalent to [u8; 16]
//     let result: String = format!("{:x}", md5::Md5::digest("Hello World"));
//     let result2: String = format!("{:x}", md5::Md5::digest("Helloo World"));

//     println!("hey {}, ||| {}", result, result2);
// }
