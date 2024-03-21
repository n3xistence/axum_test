use axum::{extract::Path, http::StatusCode, response::Json};
use serde_json::{json, Value};

use crate::models::structs::User;

pub async fn get_all_users() -> Json<Value> {
    let user = User::new(String::from("Alice"), 1);

    Json(json!(user))
}

pub async fn get_user_by_id(Path(id): Path<u32>) -> Result<Json<Value>, StatusCode> {
    let users = vec![
        User::new(String::from("Alice"), 1),
        User::new(String::from("Bob"), 2),
    ];

    match users.into_iter().find(|user| user.id == id) {
        Some(user) => Ok(Json(json!(user))),
        None => Err(StatusCode::NOT_FOUND),
    }
}
