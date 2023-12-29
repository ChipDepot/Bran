mod contexter;
mod receptor;

use std::path::PathBuf;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::ServeFile;

pub(crate) fn main_router() -> Router {
    Router::new()
        .route("/:app", post(receptor::recieve_objective))
        .route("/:app", get(contexter::get_application))
}

pub(crate) fn extras_router() -> Router {
    Router::new().route_service(
        "/favicon.ico",
        ServeFile::new(PathBuf::from("assets/favicon.ico")),
    )
}
