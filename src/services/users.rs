use axum::Json;

use crate::models::user::User;

pub async fn create_user() -> Json<serde_json::Value> {
    Json(serde_json::json!(User {
        id: 1,
        username: String::from("lalo"),
        email: String::from("lalo.a.camarena@gmail.com"),
        password: String::from("password"),
    }))
}
