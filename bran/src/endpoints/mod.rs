mod contexter;
mod receptor;

use std::path::PathBuf;

use axum::{
    routing::{get, post, put},
    Router,
};
use tower_http::services::ServeFile;

pub(crate) fn main_router() -> Router {
    Router::new()
        .route("/:app", put(receptor::update_state))
        .route("/:app", post(receptor::recieve_objective))
        .route("/:app", get(contexter::get_application))
}

pub(crate) fn directives_router() -> Router {
    Router::new()
        .route("/:app/:loc", post(receptor::recieve_addition_directive))
        .route("/:app/:loc", post(receptor::recieve_reconfig_directive))
        .route("/:app/:loc", post(receptor::recieve_restart_directive))
}

pub(crate) fn extras_router() -> Router {
    Router::new().route_service(
        "/favicon.ico",
        ServeFile::new(PathBuf::from("assets/favicon.ico")),
    )
}
