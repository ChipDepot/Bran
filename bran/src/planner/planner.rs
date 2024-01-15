use std::sync::{Arc, Mutex};

use anyhow::{bail, Result};
use uuid::Uuid;

use crate::aggregator::ApplicationRegister;

use starduck::{AdditionOrder, ReconfigureOrder, RestartOrder};
use starduck::{Application, Location, Status};

struct ProblemInfo {
    location_key: String,
    data_requirement_key: String,
}

enum Action {
    Addition,
    Restart(Uuid),
    Reconfigure(Uuid),
}

pub fn planner(register: Arc<Mutex<ApplicationRegister>>) {
    loop {}
}

fn find_problems(location_key: &str, location: &Location) -> Vec<(Action, ProblemInfo)> {
    let mut report = Vec::new();

    if location.locations.is_empty() {
        let nc_data_req = location
            .data_requirements
            .iter()
            .filter(|(_, data)| data.status != Status::Coherent)
            .collect::<Vec<_>>();

        for (data_key, data_req) in nc_data_req {
            if data_req.count < data_req.components.len() {
                report.push((
                    Action::Addition,
                    ProblemInfo {
                        location_key: location_key.to_string(),
                        data_requirement_key: data_key.to_string(),
                    },
                ))
            }
        }
    } else if location.data_requirements.is_empty() {
        for (key, i_loc) in &location.locations {
            report.extend(find_problems(key, i_loc));
        }
    }

    report
}
