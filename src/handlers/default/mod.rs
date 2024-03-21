use axum::response::Json;
use serde_json::{json, Value};

pub async fn status() -> Json<Value> {
    Json(json!({ "status": "operational", "code": 200 }))
}
