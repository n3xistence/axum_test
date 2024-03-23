use axum::{http::StatusCode, response::Json, Extension};
use dotenv::dotenv;
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::models::entities::User;
use sqlx::mysql::MySqlPool;

pub async fn get_all_users(
    Extension(pool): Extension<MySqlPool>,
) -> Result<Json<Value>, StatusCode> {
    dotenv().ok();

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

                    map.insert("id".to_string(), json!(user.id));
                    map.insert("name".to_string(), json!(user.name));
                    map.insert("email".to_string(), json!(user.email));

                    map
                })
                .collect();

            Ok(Json(json!(users_json)))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn multiple_insert_user(
    Extension(pool): Extension<MySqlPool>,
    Json(users): Json<Vec<User>>,
) -> StatusCode {
    let mut insert_statements: Vec<String> = vec![];

    for user in users.into_iter() {
        let insert_statement = format!(
            "({}, '{}', '{}')",
            match user.id {
                Some(id) => id.to_string(),
                None => String::from("NULL"),
            },
            user.name,
            user.email
        );

        insert_statements.push(insert_statement);
    }

    let base_query = String::from("INSERT INTO user(id, name, email) VALUES");

    let query = format!("{} {}", base_query, insert_statements.join(","));

    let res = sqlx::query(&query).execute(&pool).await;

    match res {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::CONFLICT,
    }
}

pub async fn single_insert_user(
    Extension(pool): Extension<MySqlPool>,
    Json(user): Json<User>,
) -> StatusCode {
    let base_query = String::from("INSERT INTO user(id, name, email) VALUES");
    let insert_statement = String::from(format!(
        "({}, '{}', '{}')",
        match user.id {
            Some(id) => id.to_string(),
            None => String::from("NULL"),
        },
        user.name,
        user.email
    ));

    let query = format!("{} {}", base_query, insert_statement);

    let res = sqlx::query(&query).execute(&pool).await;

    match res {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::CONFLICT,
    }
}
