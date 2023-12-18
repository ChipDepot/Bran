mod contexter;
mod receptor;

use axum::{
    routing::{get, post},
    Router,
};

pub(crate) fn router() -> Router {
    Router::new()
        .route("/:app", post(receptor::recieve_objective))
        .route("/:app", get(contexter::get_application))
}
