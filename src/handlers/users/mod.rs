use axum::{extract::Path, http::StatusCode, response::Json};
use serde_json::{json, Value};

use crate::models::entities::User;

pub async fn get_all_users() -> Json<Value> {
    let user = User::new(String::from("Alice"), Some(1), String::from("email"));

    Json(json!(user))
}

pub async fn get_user_by_id(Path(id): Path<u32>) -> Result<Json<Value>, StatusCode> {
    let users = vec![
        User::new(String::from("Alice"), Some(1), String::from("email")),
        User::new(String::from("Bob"), Some(2), String::from("email")),
    ];

    match users.into_iter().find(|user| match user.id {
        Some(user_id) => user_id == id,
        None => false,
    }) {
        Some(user) => Ok(Json(json!(user))),
        None => Err(StatusCode::NOT_FOUND),
    }
}
