use axum::Router;
use crate::db::Db;

pub mod user_routes;

pub fn routes() -> Router<Db> {
    Router::new()
        .merge(user_routes::routes())
}