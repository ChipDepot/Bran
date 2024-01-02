use std::collections::HashMap;
use std::net::SocketAddr;

use serde::Deserialize;
use starduck::Application;
use url::Url;

#[derive(Deserialize, Clone)]
pub struct ApplicationRegister {
    pub apps: HashMap<String, Application>,
}

impl ApplicationRegister {
    pub fn new() -> Self {
        // Initialize Register
        ApplicationRegister {
            apps: HashMap::new(),
        }
    }
}
