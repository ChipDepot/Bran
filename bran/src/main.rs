mod aggregator;
mod endpoints;
mod planner;

#[macro_use]
extern crate log;

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use axum::{Extension, Router};
use planner::Planner;
use tokio::net::TcpListener;

use aggregator::ApplicationRegister;

use starduck::utils::PORT;

#[tokio::main]
async fn main() {
    env_logger::init();

    starduck::utils::load_env(None);

    // Locate the space to handle the objective apps
    let app_aggregator = ApplicationRegister::new();
    let state_axum = Arc::new(Mutex::new(app_aggregator));
    let state_planner = Arc::clone(&state_axum);

    tokio::spawn(async move {
        let port = starduck::utils::get(PORT).unwrap_or(8014);

        let app = Router::new()
            .nest("/", endpoints::extras_router())
            .nest("/apps", endpoints::main_router())
            .nest("/directives", endpoints::directives_router())
            .layer(Extension(state_axum));

        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        let tcp_listener = TcpListener::bind(&addr).await.unwrap_or_else(|e| {
            error!("Could not start server: {e}");
            std::process::exit(-1);
        });

        info!("Initializing server at {}", &addr);

        axum::serve(
            tcp_listener,
            app.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await
        .unwrap_or_else(|e| {
            error!("Could not start server: {e}");
            std::process::exit(-1);
        });
    });

    let mut planner = Planner::new(state_planner);

    loop {
        planner.watch_over().await;
    }
}
