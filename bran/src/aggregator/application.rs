use std::{collections::HashMap, sync::Arc};

use serde::Deserialize;
use serde_json::json;
use starduck::application::Application;

use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use tokio::sync::Mutex;

#[derive(Deserialize, Clone)]
pub struct ApplicationRegister {
    apps: HashMap<String, Application>,
}

impl ApplicationRegister {
    pub fn new() -> Self {
        ApplicationRegister {
            apps: HashMap::new(),
        }
    }
}

pub async fn register_application(
    Extension(app_reg): Extension<Arc<Mutex<ApplicationRegister>>>,
    Path(app_name): Path<String>,
    Json(application): Json<Application>,
) -> Response {
    app_reg
        .lock()
        .await
        .apps
        .insert(app_name.clone(), application.clone());
    info!("{} was added to the register", app_name.clone());

    (StatusCode::OK).into_response()
}

pub async fn get_application(
    Extension(app_reg): Extension<Arc<Mutex<ApplicationRegister>>>,
    Path(app_name): Path<String>,
) -> Response {
    match app_reg.lock().await.apps.get(&app_name) {
        Some(k) => {
            let json_response = Json(k.clone());
            return (StatusCode::OK, json_response).into_response();
        }
        None => (
            StatusCode::NOT_FOUND,
            Json(json!({"msg": format!("{} not found", &app_name)})),
        )
            .into_response(),
    }
}
