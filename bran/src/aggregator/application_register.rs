use std::collections::HashMap;

use serde::Deserialize;
use starduck::{Application, Directives};

type AppName = String;
type LocationKey = String;

#[derive(Deserialize, Clone)]
pub struct ApplicationRegister {
    pub apps: HashMap<AppName, Application>,
    pub directives: HashMap<AppName, HashMap<LocationKey, Directives>>,
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
