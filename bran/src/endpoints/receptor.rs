use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use serde_json::json;
use starduck::{AdditionOrder, Application, Directives, ReconfigureOrder, RestartOrder};

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

    let mut guard = app_reg.lock().unwrap();

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

    let mut guard = app_reg.lock().unwrap();

    if let None = guard.apps.get(&app_name) {
        error!("Application is not in the register");
        return (StatusCode::NOT_FOUND).into_response();
    }

    guard.apps.insert(app_name.clone(), application.clone());

    info!("{}'s state was updated", app_name.clone());

    (StatusCode::OK).into_response()
}

pub async fn recieve_addition_directive(
    Extension(app_reg): Extension<Arc<Mutex<ApplicationRegister>>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Path((app_name, location)): Path<(String, String)>,
    Json(order): Json<AdditionOrder>,
) -> Response {
    info!("POST for {} request from {}", app_name, addr);

    let reg = app_reg.lock().unwrap().clone();

    match reg.apps.get(&app_name) {
        // The application exists on the register
        Some(application) => match (
            application.locations.get(&location),
            reg.directives.get(&app_name),
        ) {
            // The location exists and there are directives registered for it
            (Some(_), Some(directives)) => {
                // There is already a directive in the location
                if let Some(_) = directives.get(&location) {
                    app_reg
                        .lock()
                        .unwrap()
                        .directives
                        .get_mut(&app_name)
                        .unwrap()
                        .get_mut(&location)
                        .unwrap()
                        .addition = Some(order);

                    let msg = format!(
                        "Updated addition directive in {} in app {}",
                        &location, &app_name
                    );
                    info!("{}", msg);
                    return (StatusCode::OK, Json(json!({"msg": msg}))).into_response();
                }

                // There is not a directive in the location. Have to create a new register
                let mut directive = Directives::new();
                directive.addition = Some(order);

                app_reg
                    .lock()
                    .unwrap()
                    .directives
                    .get_mut(&app_name)
                    .unwrap()
                    .insert(location.clone(), directive);

                let msg = format!(
                    "Added addition directive in {} in app {}",
                    &location, &app_name
                );
                info!("{}", msg);
                return (StatusCode::OK, Json(json!({"msg": msg}))).into_response();
            }
            // The location exists but there are no directives registerd for it
            (Some(_), None) => {
                let mut directive_hash = HashMap::new();
                let mut directive = Directives::new();
                directive.addition = Some(order);

                directive_hash.insert(location.clone(), directive);

                app_reg
                    .lock()
                    .unwrap()
                    .directives
                    .insert(app_name.clone(), directive_hash);

                let msg = format!(
                    "Added addition directive in {} in app {}",
                    &location, &app_name
                );
                info!("{}", msg);
                return (StatusCode::OK, Json(json!({"msg": msg}))).into_response();
            }
            // The location doesn't exist in the application
            (None, _) => {
                let msg = format!(
                    "Couldn't find location {} in  application {}",
                    location, app_name
                );
                error!("{}", msg);
                return (StatusCode::NOT_FOUND, Json(json!({"msg": msg}))).into_response();
            }
        },
        // The application doesn't exist on the register
        None => {
            let msg = "Couldn't find application in register";
            error!("{}", msg);
            return (StatusCode::NOT_FOUND, Json(json!({"msg": msg}))).into_response();
        }
    }
}

pub async fn recieve_reconfig_directive(
    Extension(app_reg): Extension<Arc<Mutex<ApplicationRegister>>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Path((app_name, location)): Path<(String, String)>,
    Json(order): Json<ReconfigureOrder>,
) -> Response {
    info!("POST for {} request from {}", app_name, addr);

    let reg = app_reg.lock().unwrap().clone();

    match reg.apps.get(&app_name) {
        // The application exists on the register
        Some(application) => match (
            application.locations.get(&location),
            reg.directives.get(&app_name),
        ) {
            // The location exists and there are directives registered for it
            (Some(_), Some(directives)) => {
                // There is already a directive in the location
                if let Some(_) = directives.get(&location) {
                    app_reg
                        .lock()
                        .unwrap()
                        .directives
                        .get_mut(&app_name)
                        .unwrap()
                        .get_mut(&location)
                        .unwrap()
                        .reconfig = Some(order);

                    let msg = format!(
                        "Updated reconfig directive in {} in app {}",
                        &location, &app_name
                    );
                    info!("{}", msg);
                    return (StatusCode::OK, Json(json!({"msg": msg}))).into_response();
                }

                // There is not a directive in the location. Have to create a new register
                let mut directive = Directives::new();
                directive.reconfig = Some(order);

                app_reg
                    .lock()
                    .unwrap()
                    .directives
                    .get_mut(&app_name)
                    .unwrap()
                    .insert(location.clone(), directive);

                let msg = format!(
                    "Added reconfig directive in {} in app {}",
                    &location, &app_name
                );
                info!("{}", msg);
                return (StatusCode::OK, Json(json!({"msg": msg}))).into_response();
            }
            // The location exists but there are no directives registerd for it
            (Some(_), None) => {
                let mut directive_hash = HashMap::new();
                let mut directive = Directives::new();
                directive.reconfig = Some(order);

                directive_hash.insert(location.clone(), directive);

                app_reg
                    .lock()
                    .unwrap()
                    .directives
                    .insert(app_name.clone(), directive_hash);

                let msg = format!(
                    "Added reconfig directive in {} in app {}",
                    &location, &app_name
                );
                info!("{}", msg);
                return (StatusCode::OK, Json(json!({"msg": msg}))).into_response();
            }
            // The location doesn't exist in the application
            (None, _) => {
                let msg = format!(
                    "Couldn't find location {} in  application {}",
                    location, app_name
                );
                error!("{}", msg);
                return (StatusCode::NOT_FOUND, Json(json!({"msg": msg}))).into_response();
            }
        },
        // The application doesn't exist on the register
        None => {
            let msg = "Couldn't find application in register";
            error!("{}", msg);
            return (StatusCode::NOT_FOUND, Json(json!({"msg": msg}))).into_response();
        }
    }
}

pub async fn recieve_restart_directive(
    Extension(app_reg): Extension<Arc<Mutex<ApplicationRegister>>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Path((app_name, location)): Path<(String, String)>,
    Json(order): Json<RestartOrder>,
) -> Response {
    info!("POST for {} request from {}", app_name, addr);

    let reg = app_reg.lock().unwrap().clone();

    match reg.apps.get(&app_name) {
        // The application exists on the register
        Some(application) => match (
            application.locations.get(&location),
            reg.directives.get(&app_name),
        ) {
            // The location exists and there are directives registered for it
            (Some(_), Some(directives)) => {
                // There is already a directive in the location
                if let Some(_) = directives.get(&location) {
                    app_reg
                        .lock()
                        .unwrap()
                        .directives
                        .get_mut(&app_name)
                        .unwrap()
                        .get_mut(&location)
                        .unwrap()
                        .restart = Some(order);

                    let msg = format!(
                        "Updated restart directive in {} in app {}",
                        &location, &app_name
                    );
                    info!("{}", msg);
                    return (StatusCode::OK, Json(json!({"msg": msg}))).into_response();
                }

                // There is not a directive in the location. Have to create a new register
                let mut directive = Directives::new();
                directive.restart = Some(order);

                app_reg
                    .lock()
                    .unwrap()
                    .directives
                    .get_mut(&app_name)
                    .unwrap()
                    .insert(location.clone(), directive);

                let msg = format!(
                    "Added restart directive in {} in app {}",
                    &location, &app_name
                );
                info!("{}", msg);
                return (StatusCode::OK, Json(json!({"msg": msg}))).into_response();
            }
            // The location exists but there are no directives registerd for it
            (Some(_), None) => {
                let mut directive_hash = HashMap::new();
                let mut directive = Directives::new();
                directive.restart = Some(order);

                directive_hash.insert(location.clone(), directive);

                app_reg
                    .lock()
                    .unwrap()
                    .directives
                    .insert(app_name.clone(), directive_hash);

                let msg = format!(
                    "Added restart directive in {} in app {}",
                    &location, &app_name
                );
                info!("{}", msg);
                return (StatusCode::OK, Json(json!({"msg": msg}))).into_response();
            }
            // The location doesn't exist in the application
            (None, _) => {
                let msg = format!(
                    "Couldn't find location {} in  application {}",
                    location, app_name
                );
                error!("{}", msg);
                return (StatusCode::NOT_FOUND, Json(json!({"msg": msg}))).into_response();
            }
        },
        // The application doesn't exist on the register
        None => {
            let msg = "Couldn't find application in register";
            error!("{}", msg);
            return (StatusCode::NOT_FOUND, Json(json!({"msg": msg}))).into_response();
        }
    }
}
