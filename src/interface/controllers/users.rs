use axum::{Router, routing::{get, post}};

use crate::services::users as users_service;
use crate::interface::html_templates::users as users_templates;

pub fn create_controller() -> Router {
    Router::new()
        .route("/", post(users_service::create_user))
        .route("/login", get(login))
}

pub async fn login() -> users_templates::Login {
    users_templates::Login {}
}
