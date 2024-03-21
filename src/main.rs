mod handlers;
mod models;

use handlers::{database, default, users};

use axum::http::{HeaderValue, Method};
use axum::{routing::get, Router};
use dotenv::dotenv;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cors_layer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap());

    let address = dotenv::var("ADDRESS").unwrap();
    let port = dotenv::var("PORT").unwrap();

    let binding = format!("{}:{}", address, port);

    let app = Router::new()
        .route("/", get(default::not_implemented))
        .route("/status", get(default::status))
        .route("/user", get(users::get_all_users))
        .route("/user/:id", get(users::get_user_by_id))
        .route("/users", get(database::get_all_users))
        .layer(cors_layer);

    let listener = tokio::net::TcpListener::bind(binding).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
