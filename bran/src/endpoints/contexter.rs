use std::{net::SocketAddr, sync::Arc};

use serde_json::json;
use tokio::sync::Mutex;

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

    match app_reg.lock().await.apps.get(&app_name) {
        Some(k) => {
            let json_response = Json(k.clone());
            info!("{} info sent to {}", app_name, addr);
            return (StatusCode::OK, json_response).into_response();
        }
        None => {
            warn!(
                "Missing {} context. Use lexical client to set state",
                app_name
            );
            (
                StatusCode::NOT_FOUND,
                Json(json!({"msg": format!("{} context not found. Use lexical client to set state", &app_name)})),
            )
                .into_response()
        }
    }
}
