mod aggregator;
mod endpoints;

#[macro_use]
extern crate log;

use axum::{Extension, Router, Server};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;

use aggregator::application::ApplicationRegister;

#[tokio::main]
async fn main() {
    env_logger::init();

    let app_aggregator = ApplicationRegister::new();
    let shared_state = Arc::new(Mutex::new(app_aggregator));
    info!("Initialized Shared Space");

    let app = Router::new()
        .nest("/api", endpoints::router())
        .layer(Extension(shared_state));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8014));

    info!("Starting server at {}", &addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
