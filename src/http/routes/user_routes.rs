use axum::{Router, routing::{get,post}};
use crate::db::Db;
use crate::http::controllers::user_controller;

pub fn routes() -> Router<Db> {
    Router::new()
        .route("/users", post(user_controller::store))
        .route("/users", get(user_controller::index))
}