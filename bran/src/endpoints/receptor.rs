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

    let mut guard = app_reg.lock().await;

    if let Some(_) = guard.apps.get(&app_name) {
        error!("Application already registered");
        return (StatusCode::BAD_REQUEST).into_response();
    }

    guard.apps.insert(app_name.clone(), application.clone());

    info!("{} was added to the register", app_name.clone());

    (StatusCode::OK).into_response()
}

pub async fn update_state(
    Extension(app_reg): Extension<Arc<Mutex<ApplicationRegister>>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Path(app_name): Path<String>,
    Json(application): Json<Application>,
) -> Response {
    info!("PUT for {} request from {}", app_name, addr);

    let mut guard = app_reg.lock().await;

    if let None = guard.apps.get(&app_name) {
        error!("Application is not in the register");
        return (StatusCode::BAD_REQUEST).into_response();
    }

    guard.apps.insert(app_name.clone(), application.clone());

    info!("{}'s state was updated", app_name.clone());

    (StatusCode::OK).into_response()
}
