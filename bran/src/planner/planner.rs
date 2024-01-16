use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};

use uuid::Uuid;

use crate::aggregator::ApplicationRegister;
use crate::planner::build_order::BuildOrder;
use crate::planner::make_request::MakeRequest;

use starduck::{Location, Status};

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct ProblemInfo {
    pub location_key: String,
    pub data_requirement_key: String,
    pub device_uuid: Option<Uuid>,
}

impl ProblemInfo {
    pub fn new(location_key: &str, data_key: &str, device_uuid: Option<Uuid>) -> Self {
        Self {
            location_key: location_key.to_string(),
            data_requirement_key: data_key.to_string(),
            device_uuid,
        }
    }
}

enum Action {
    Addition(usize),
    Restart,
    Reconfigure,
}

pub struct Planner {
    register: Arc<Mutex<ApplicationRegister>>,
    problem_action: HashMap<ProblemInfo, Action>,
}

impl Planner {
    const DOTHING: &str = "dothing:8050";

    pub fn new(register: Arc<Mutex<ApplicationRegister>>) -> Self {
        Self {
            register,
            problem_action: HashMap::new(),
        }
    }

    async fn execute_actions(&self) {
        let applications = self
            .register
            .lock()
            .unwrap()
            .apps
            .iter()
            .filter_map(|(_, app)| {
                if app.status != Status::Coherent {
                    return Some(app);
                } else {
                    None
                }
            })
            .cloned()
            .collect::<Vec<_>>();

        let target = env::var(Self::DOTHING).unwrap_or("localhost:8050".to_owned());

        for app in applications {
            if let Some(directives) = self.register.lock().unwrap().directives.get(&app.name) {
                for problem in self.find_problems("root", &app.locations) {
                    match problem {
                        (Action::Addition(count), p) => {
                            if let Some(order) =
                                directives.get(&p.location_key).map(|d| d.addition.clone())
                            {
                                let mut mod_order = order.clone();

                                if let Err(e) = mod_order.build_order(&p) {
                                    error!("{e}");
                                    continue;
                                }

                                for _ in 0..count {
                                    if let Err(e) = mod_order.make_request(&target).await {
                                        error!("{e}");
                                        continue;
                                    };
                                }
                            }

                            warn!(
                                "No Addition directive for {} in app {}!",
                                &p.location_key, &app.name
                            );
                        }
                        (Action::Reconfigure, p) => {
                            if let Some(order) =
                                directives.get(&p.location_key).map(|d| d.reconfig.clone())
                            {
                                let mut mod_order = order.clone();
                                mod_order.uuid = Some(p.device_uuid.unwrap());

                                if let Err(e) = mod_order.make_request(&target).await {
                                    error!("{e}");
                                    continue;
                                };
                            }

                            warn!(
                                "No Reconfigure directive for {} in app {}!",
                                &p.location_key, &app.name
                            );
                        }
                        (Action::Restart, p) => {
                            if let Some(order) =
                                directives.get(&p.location_key).map(|d| d.restart.clone())
                            {
                                let mut mod_order = order.clone();
                                mod_order.uuid = Some(p.device_uuid.unwrap());

                                if let Err(e) = mod_order.make_request(&target).await {
                                    error!("{e}");
                                    continue;
                                };
                            }

                            warn!(
                                "No Restart directive for {} in app {}!",
                                &p.location_key, &app.name
                            );
                        }
                    }
                }
            } else {
                warn!("No directives for {}!", app.name);
            }
        }
    }

    fn find_problems(&self, location_key: &str, location: &Location) -> Vec<(Action, ProblemInfo)> {
        let mut report = Vec::new();

        if location.locations.is_empty() {
            let nc_data_req = location
                .data_requirements
                .iter()
                .filter(|(_, data)| data.status != Status::Coherent)
                .collect::<Vec<_>>();

            for (data_key, data_req) in nc_data_req {
                let req_count = data_req.components.len();

                // Missing services, has to add more
                if data_req.count < req_count {
                    let missing_count = req_count - data_req.count;
                    let problem_info = ProblemInfo::new(location_key, data_key, None);
                    report.push((Action::Addition(missing_count), problem_info));

                //
                } else if data_req.count >= req_count {
                    for comp in data_req.components.clone() {
                        let problem_info = ProblemInfo::new(location_key, data_key, comp.uuid);

                        match self.problem_action.get(&problem_info) {
                            Some(Action::Restart) => {
                                report.push((Action::Reconfigure, problem_info))
                            }
                            Some(_) => report.push((Action::Restart, problem_info)),
                            None => report.push((Action::Addition(1), problem_info)),
                        }
                    }
                }
            }
        } else if location.data_requirements.is_empty() {
            for (key, i_loc) in &location.locations {
                report.extend(self.find_problems(key, i_loc));
            }
        }
        report
    }
}
