use actix_web::{web, App, HttpServer};
use url_shortner::{middleware::analytics::Analytics, state::app_state::AppState};
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialise database core here so that it can be passed in application states
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState::new(String::from(
                "redis://default:i6rA6vZFaqy6tqXiP2Bcz9xOhY9C3WKT@redis-15268.c305.ap-south-1-1.ec2.cloud.redislabs.com:15268",
            ))))
            .service(
                web::scope("get")
                    .wrap(Analytics)
                    .route("/{hash}", web::get().to(handlers::api::get_long_url)),
            )
            .route("/shorten", web::post().to(handlers::api::shorten_url))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
