use anyhow::{bail, Result};
use async_trait::async_trait;
use reqwest::Client;

use starduck::{AdditionOrder, ReconfigureOrder, RestartOrder};

#[async_trait]
pub trait MakeRequest {
    const ENDPOINT: &'static str;

    async fn make_request(&self, target: &str) -> Result<()>;
}

#[async_trait]
impl MakeRequest for AdditionOrder {
    const ENDPOINT: &'static str = "/addition";

    async fn make_request(&self, target: &str) -> Result<()> {
        let url = format!("{}{}", target, Self::ENDPOINT);

        let client = Client::new();
        match client.post(url).json(&self).send().await {
            Ok(_) => return Ok(()),
            Err(e) => bail!("{e}"),
        }
    }
}

#[async_trait]
impl MakeRequest for RestartOrder {
    const ENDPOINT: &'static str = "/restart";

    async fn make_request(&self, target: &str) -> Result<()> {
        let url = format!("{}{}", target, Self::ENDPOINT);

        let client = Client::new();
        match client.post(url).json(&self).send().await {
            Ok(_) => return Ok(()),
            Err(e) => bail!("{e}"),
        }
    }
}

#[async_trait]
impl MakeRequest for ReconfigureOrder {
    const ENDPOINT: &'static str = "/reconfig/http";

    async fn make_request(&self, target: &str) -> Result<()> {
        let url = format!("{}{}", target, Self::ENDPOINT);

        let client = Client::new();
        match client.post(url).json(&self).send().await {
            Ok(_) => return Ok(()),
            Err(e) => bail!("{e}"),
        }
    }
}
