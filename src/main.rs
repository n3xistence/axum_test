use axum::{response::Json, routing::get, Router};
use serde_json::{json, Value};

mod handlers;
mod models;

use handlers::users;

async fn status() -> Json<Value> {
    Json(json!({ "status": "operational", "code": 200 }))
}

#[tokio::main]
async fn main() {
    let address = String::from("0.0.0.0:3000");

    let app = Router::new()
        .route("/", get(status))
        .route("/user", get(users::get_all_users))
        .route("/user/:id", get(users::get_user_by_id));

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
