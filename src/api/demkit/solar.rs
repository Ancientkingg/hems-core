use num_complex::Complex;
use serde::Deserialize;

use super::{init, parse_complex_str, ApiError, Commodities, BASE_URL, CLIENT};

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct SolarProperties {
    pub name: String,
    #[serde(rename = "timeBase")]
    pub time_base: i64,
    #[serde(rename = "timeOffset")]
    pub time_offset: i64,
    pub devtype: String,
    pub commodities: Vec<String>,
    #[serde(rename = "strictComfort")]
    pub strict_comfort: bool,
    pub consumption: Commodities,
    pub size: f64,
    pub efficiency: f64,
    pub inclination: f64,
    pub azimuth: f64,
    #[serde(rename = "onOffDevice")]
    pub on_off_device: bool,
    #[serde(rename = "originalConsumption")]
    pub original_consumption: Commodities,

    pub electricity_consumption: Option<Complex<f64>>,
}

pub async fn get_solar_properties(house_id: u32) -> Result<SolarProperties, ApiError> {
    let client = CLIENT.get_or_init(init);

    let url = format!("{}/call/PV-House-{house_id}/getProperties", BASE_URL);

    let response = client.get(url).send().await?;

    let mut response_body = response.json::<SolarProperties>().await?;

    response_body.electricity_consumption =
        Some(parse_complex_str(&response_body.original_consumption.electricity)?);

    Ok(response_body)
}

pub async fn set_solar_state(house_id: u32, state: bool) -> Result<(), ApiError> {
    let client = CLIENT.get_or_init(init);

    let state = if state { "True" } else { "False" };

    let url = format!("{}/set/PV-House-{house_id}/onOffDevice/{state}", BASE_URL);

    client.get(url).send().await?;

    Ok(())
}
