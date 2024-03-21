mod handlers;
mod models;

use handlers::{default, users};

use axum::{routing::get, Router};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let address = dotenv::var("ADDRESS").unwrap();
    let port = dotenv::var("PORT").unwrap();

    let binding = format!("{}:{}", address, port);

    let app = Router::new()
        .route("/", get(default::status))
        .route("/user", get(users::get_all_users))
        .route("/user/:id", get(users::get_user_by_id));

    let listener = tokio::net::TcpListener::bind(binding).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
