use std::env::{self, VarError};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::process::exit;

use anyhow::Result;
use dotenv::dotenv;
use log::info;

use super::file_handler;

const DEFAULT_ENV_TEMPLATE_PATH: &str = "config/default.env";
const DEFAULT_ENV_PATH: &str = ".env";

fn create_default_env_file() {
    // Get the template .env file path and open the file
    let path = file_handler::get_absolute_path(DEFAULT_ENV_TEMPLATE_PATH)
        .unwrap_or_else(|err| panic!("{}", err));
    let template_env_file = File::open(&path).unwrap();

    // Create the new .env file for user edition
    let new_env_path =
        file_handler::get_absolute_path(DEFAULT_ENV_PATH).unwrap_or_else(|err| panic!("{}", err));
    let new_env = File::create(&new_env_path).unwrap();

    info!("Creating .env at {}", &new_env_path.to_str().unwrap());

    // Create reader and writer buffers for the files
    let mut reader = BufReader::new(template_env_file);
    let mut writer = BufWriter::new(new_env);

    // Copy the contents from the template to the new file
    std::io::copy(&mut reader, &mut writer).unwrap_or_else(|err| panic!("{}", err));

    info!("Created .env file from default template");
}

pub fn load_env(path: Option<&str>) {
    let env_path = path.unwrap_or(DEFAULT_ENV_PATH);
    let env_path =
        file_handler::get_absolute_path(env_path).unwrap_or_else(|err| panic!("{}", err));

    // First, we need to check if the an .env file exists.
    if !file_handler::file_exists(&env_path) {
        warn!(".env file is missing");
        create_default_env_file();
        exit(0);
    };

    info!("Found .env file");
    info!("Loading .env");
    dotenv().ok();
}

pub fn get<T: std::str::FromStr>(key: &str) -> Result<T> {
    let var = env::var(key)?;
    match var.parse::<T>() {
        Ok(k) => Ok(k),
        Err(_) => Err(VarError::NotPresent.into()),
    }
}
