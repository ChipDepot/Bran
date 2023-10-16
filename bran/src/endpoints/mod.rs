mod contexter;
mod receptor;

use std::sync::Arc;

use super::aggregator::application;

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Router,
};
use log::info;

pub(crate) fn router() -> Router {
    Router::new().route(
        "/:app",
        post(application::register_application).get(application::get_application),
    )
}
