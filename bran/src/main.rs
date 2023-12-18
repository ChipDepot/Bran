mod aggregator;
mod endpoints;

#[macro_use]
extern crate log;

use axum::{Extension, Router};
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::{net::TcpListener, sync::Mutex, time::sleep};

use aggregator::application::ApplicationRegister;

#[tokio::main]
async fn main() {
    env_logger::init();

    // Locate the space to handle the objective apps
    let app_aggregator = ApplicationRegister::new();
    let shared_state = Arc::new(Mutex::new(app_aggregator));
    info!("Initialized Shared Space");

    tokio::spawn(async {
        let app = Router::new()
            .nest("/", endpoints::router())
            .layer(Extension(shared_state));
        let addr = SocketAddr::from(([0, 0, 0, 0], 8014));
        let tcp_listener = TcpListener::bind(&addr).await.unwrap();

        info!("Initializing server at {}", &addr);
        axum::serve(
            tcp_listener,
            app.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await
        .unwrap();
    });

    loop {
        sleep(Duration::from_secs(60)).await;
    }
}
