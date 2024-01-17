use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::Mutex;

use serde_json::json;

use axum::{
    extract::{ConnectInfo, Json, Path},
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension,
};

use crate::ApplicationRegister;

pub async fn get_application(
    Extension(app_reg): Extension<Arc<Mutex<ApplicationRegister>>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Path(app_name): Path<String>,
) -> Response {
    info!("Get for {} request from {}", app_name, addr);

    let m_app_reg = app_reg.lock().unwrap();

    if let Some(app) = m_app_reg.apps.get(&app_name) {
        let json_response = Json(app.clone());

        info!("{} info sent to {}", app_name, addr);
        return (StatusCode::OK, json_response).into_response();
    }

    warn!("Missing {app_name} context. Use lexical client to set state");

    let err_msg = format!("{app_name} context not found. Use lexical client to set state");
    (StatusCode::NOT_FOUND, Json(json!({"msg": err_msg}))).into_response()
}

pub async fn get_application_directives(
    Extension(app_reg): Extension<Arc<Mutex<ApplicationRegister>>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Path(app_name): Path<String>,
) -> Response {
    info!("Get for {} request from {}", app_name, addr);

    let m_app_reg = app_reg.lock().unwrap();

    if let Some(app) = m_app_reg.directives.get(&app_name) {
        let json_response = Json(app.clone());

        info!("{} info sent to {}", app_name, addr);
        return (StatusCode::OK, json_response).into_response();
    }

    warn!("Missing {app_name} context. Use lexical client to set state");

    let err_msg = format!("{app_name} context not found. Use lexical client to set state");
    (StatusCode::NOT_FOUND, Json(json!({"msg": err_msg}))).into_response()
}
