use axum::{extract::Json, response::Response, Error};
use starduck::application::Application;

pub async fn recieve_objective(request: Json<Application>) -> Response {
    todo!()
}
