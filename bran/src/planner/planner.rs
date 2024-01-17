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
    pub fn new(location_key: &str, data_key: &str, device_uuid: &Option<Uuid>) -> Self {
        Self {
            location_key: location_key.to_string(),
            data_requirement_key: data_key.to_string(),
            device_uuid: device_uuid.clone(),
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
    const DOTHING: &str = "dothing";
    const WATCHER_DELAY: &str = "watcher_delay";
    const INTERVAL: &str = "watcher_interval";

    pub fn new(register: Arc<Mutex<ApplicationRegister>>) -> Self {
        Self {
            register,
            problem_action: HashMap::new(),
        }
    }

    pub async fn watch_over(&mut self) {
        let wait = env::var(Self::WATCHER_DELAY)
            .map(|k| std::time::Duration::from_secs(k.parse().unwrap()))
            .unwrap_or(std::time::Duration::from_secs(0));

        let interval = env::var(Self::INTERVAL)
            .map(|k| std::time::Duration::from_secs(k.parse().unwrap()))
            .unwrap_or(std::time::Duration::from_secs(120));

        std::thread::sleep(wait);

        loop {
            info!("Starting Planner Execution");

            self.execute_actions().await;

            std::thread::sleep(interval);
        }
    }

    async fn execute_actions(&mut self) {
        let applications = self
            .register
            .lock()
            .unwrap()
            .apps
            .iter()
            .filter_map(|(_, app)| {
                if app.status != Status::Coherent && app.status != Status::Uninitialized {
                    return Some(app);
                } else {
                    None
                }
            })
            .cloned()
            .collect::<Vec<_>>();

        let target = env::var(Self::DOTHING).unwrap_or("http://dothing:8050".to_owned());

        for app in applications {
            info!("Checking Application {}", &app.name);

            if let Some(directives) = self.register.lock().unwrap().directives.get(&app.name) {
                for problem in self.find_problems("root", &app.locations) {
                    match problem {
                        (Action::Addition(count), p) => {
                            if let Some(Some(order)) =
                                directives.get(&p.location_key).map(|d| d.addition.clone())
                            {
                                info!("Executing Addition order");

                                for i in 1..=count {
                                    let mut mod_order = order.clone();

                                    info!(
                                        "Building addition order {} out of {} from {:?}",
                                        i, count, &p
                                    );
                                    if let Err(e) = mod_order.build_order(&p) {
                                        error!("{e}");
                                        continue;
                                    }

                                    info!("Executing  order {} out of {}", i, count);

                                    mod_order.make_request(&target).await.unwrap();

                                    // if let Err(e) = mod_order.make_request(&target).await {
                                    //     error!("{e}");
                                    //     continue;
                                    // };
                                }
                                continue;
                            }

                            warn!(
                                "No Addition directive for {} in app {}!",
                                &p.location_key, &app.name
                            );
                        }
                        (Action::Reconfigure, p) => {
                            info!("Executing Reconfigure order");

                            if let Some(Some(order)) =
                                directives.get(&p.location_key).map(|d| d.reconfig.clone())
                            {
                                self.problem_action.insert(p.clone(), Action::Reconfigure);

                                let mut mod_order = order.clone();
                                mod_order.uuid = Some(p.device_uuid.unwrap());

                                info!("Executing order: {:?}", &mod_order);

                                mod_order.make_request(&target).await.unwrap();

                                // if let Err(e) = mod_order.make_request(&target).await {
                                //     error!("{e}");
                                //     continue;
                                // };
                            }

                            warn!(
                                "No Reconfigure directive for {} in app {}!",
                                &p.location_key, &app.name
                            );
                        }
                        (Action::Restart, p) => {
                            info!("Executing Restart order");

                            if let Some(Some(order)) =
                                directives.get(&p.location_key).map(|d| d.restart.clone())
                            {
                                self.problem_action.insert(p.clone(), Action::Restart);

                                let mut mod_order = order.clone();
                                mod_order.uuid = Some(p.device_uuid.unwrap());

                                info!("Executing order: {:?}", &mod_order);

                                mod_order.make_request(&target).await.unwrap();

                                // if let Err(e) = mod_order.make_request(&target).await {
                                //     error!("{e}");
                                //     continue;
                                // };
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

            warn!(
                "Found {} data requirements with errors in {}",
                &nc_data_req.len(),
                location_key
            );

            for (data_key, data_req) in nc_data_req {
                let comp_count = data_req.components.len();

                // Missing services, has to add more
                if data_req.count > comp_count {
                    info!(
                        "Creating Addition Order for data requirement {} in {}",
                        data_key, location_key
                    );

                    let missing_count = data_req.count - comp_count;
                    let problem_info = ProblemInfo::new(location_key, data_key, &None);
                    report.push((Action::Addition(missing_count), problem_info));

                //
                } else if data_req.count <= comp_count {
                    for comp in data_req.components.clone() {
                        let problem_info = ProblemInfo::new(location_key, data_key, &comp.uuid);

                        match self.problem_action.get(&problem_info) {
                            Some(Action::Restart) => {
                                info!(
                                    "Creating Reconfigure Order for component {} in data requirement {} in {}",
                                    comp.uuid.unwrap(), data_key, location_key
                                );

                                report.push((Action::Reconfigure, problem_info))
                            }
                            Some(_) => {
                                info!(
                                    "Creating Addition Order for data requirement {} in {}",
                                    data_key, location_key
                                );
                                let problem_info = ProblemInfo::new(location_key, data_key, &None);

                                report.push((Action::Addition(1), problem_info));
                            }
                            None => {
                                info!(
                                    "Creating Restart Order for component {} data requirement {} in {}",
                                    comp.uuid.unwrap(), data_key, location_key
                                );

                                report.push((Action::Restart, problem_info));
                            }
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
