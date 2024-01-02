use std::{net::SocketAddr, sync::Arc};

use starduck::Application;
use tokio::sync::Mutex;

use axum::{
    extract::{ConnectInfo, Json, Path},
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension,
};

use crate::ApplicationRegister;

pub async fn recieve_objective(
    Extension(app_reg): Extension<Arc<Mutex<ApplicationRegister>>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Path(app_name): Path<String>,
    Json(application): Json<Application>,
) -> Response {
    info!("POST for {} request from {}", app_name, addr);

    let reg = app_reg
        .lock()
        .await
        .apps
        .insert(app_name.clone(), application.clone());

    match reg {
        Some(_) => info!("{}'s state was updated", app_name.clone()),
        None => info!("{} was added to the register", app_name.clone()),
    }

    (StatusCode::OK).into_response()
}
