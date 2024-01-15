use std::collections::HashMap;

use serde::Deserialize;
use starduck::{Application, Directives};

#[derive(Deserialize, Clone)]
pub struct ApplicationRegister {
    pub apps: HashMap<String, Application>,
    pub directives: HashMap<String, HashMap<String, Directives>>,
}

impl ApplicationRegister {
    pub fn new() -> Self {
        // Initialize Register
        ApplicationRegister {
            apps: HashMap::new(),
            directives: HashMap::new(),
        }
    }
}
