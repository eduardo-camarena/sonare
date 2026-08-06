use askama::Template;
use askama_web::WebTemplate;
use axum::{Router, routing::get};

use crate::interface::controllers::users::create_controller as users_controller;

pub fn create_app() -> Router {
    Router::new()
        .route("/", get(main_page))
        .nest("/users", users_controller())
}

#[derive(Template, WebTemplate)]
#[template(path = "index.html")]
pub struct Hello {
    name: String,
}

pub async fn main_page() -> Hello {
    Hello {
        name: String::from("lalo"),
    }
}
