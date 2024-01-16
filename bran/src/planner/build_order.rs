use anyhow::{bail, Result};

use starduck::AdditionOrder;
use uuid::Uuid;

use super::planner::ProblemInfo;

const DATAKEY: &str = "key=";

pub trait BuildOrder<T> {
    fn build_order(&mut self, t: &T) -> Result<()>;

    fn process_datakey(&mut self, req_key: &str) -> Result<Option<String>>;
}

impl BuildOrder<ProblemInfo> for AdditionOrder {
    fn build_order(&mut self, t: &ProblemInfo) -> Result<()> {
        // Add add device id from order
        self.env_vars.insert(
            "device_uuid".to_owned(),
            serde_json::Value::from(Uuid::new_v4().to_string()),
        );

        if let Some(k) = self.process_datakey(&t.data_requirement_key)? {
            self.args.push(k);
        }

        self.args.push(format!("location={}", t.location_key));
        self.args.push(format!("topic={}", t.data_requirement_key));

        Ok(())
    }

    fn process_datakey(&mut self, req_key: &str) -> Result<Option<String>> {
        let datakeys = self
            .args
            .iter()
            .cloned()
            .enumerate()
            .filter_map(|(index, s)| {
                if s.contains(DATAKEY) {
                    return Some((index, s));
                }
                None
            })
            .collect::<Vec<_>>();

        match datakeys.len() {
            1 => {
                let (index, str) = &datakeys[0];
                self.args.remove(index.clone());

                Ok(Some(str.replace(DATAKEY, format!("{}=", req_key).as_str())))
            }
            0 => Ok(None),
            _ => bail!("More than one key= string found"),
        }
    }
}
