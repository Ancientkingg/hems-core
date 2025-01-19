use std::sync::OnceLock;

use reqwest::{self, Error};

const BASE_URL: &str = "http://localhost:4000";

static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

pub fn init() -> reqwest::Client {
    reqwest::Client::new()
}

#[derive(Deserialize)]
pub struct Measurement {
    pub value: i32,
    pub unit: String,
}

pub async fn get_energy_import() -> Result<Measurement, Error> {
    let client= CLIENT.get_or_init(init);

    let url = format!("{}/energy/import", BASE_URL);

    let response = client
        .get(url)
        .send()
        .await?;

    let response_body = response.json::<Measurement>().await?;

    Ok(response_body)
}

pub async fn get_energy_export() -> Result<i32, Error> {
    todo!("")
}

pub async fn get_battery_status() -> Result<i32, Error> {
    todo!("")
}

pub async fn get_device_consumption(device_name: &str) -> Result<i32, Error> {
    todo!("")
}