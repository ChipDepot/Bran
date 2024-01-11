use std::collections::HashMap;

use serde::Deserialize;
use starduck::Application;

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
