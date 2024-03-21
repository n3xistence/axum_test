use axum::{http::StatusCode, response::Json};
use dotenv::dotenv;
use serde_json::{json, Value};
use std::collections::HashMap;

use sqlx::mysql::MySqlPoolOptions;

pub async fn get_all_users() -> Result<Json<Value>, StatusCode> {
    dotenv().ok();

    let connection_url = dotenv::var("DATABASE_URL").unwrap();

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&connection_url)
        .await
        .unwrap();

    let users = sqlx::query!("SELECT * FROM user").fetch_all(&pool).await;

    match users {
        Ok(users) => {
            if users.is_empty() {
                return Err(StatusCode::NOT_FOUND);
            }

            let users_json: Vec<HashMap<String, Value>> = users
                .into_iter()
                .map(|user| {
                    let mut map = HashMap::new();
                    // Assuming `user` has fields `id` and `name`
                    map.insert("id".to_string(), json!(user.id));
                    map.insert("name".to_string(), json!(user.name));
                    map
                })
                .collect();

            Ok(Json(json!(users_json)))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
