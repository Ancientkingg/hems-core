use num_complex::Complex;
use serde::Deserialize;

use super::{init, parse_complex_str, ApiError, Commodities, BASE_URL, CLIENT};

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ZoneProperties {
    pub temperature: f64,
    #[serde(rename = "valveHeat")]
    pub valve_heat: f64,
    #[serde(rename = "consumption")]
    pub original_consumption: Commodities,
    pub heat_consumption: Option<Complex<f64>>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ThermostatProperties {
    pub min_target_temp: f64,
    pub max_target_temp: f64,
}


pub async fn get_current_zone_temp(house_id: u32) -> Result<ZoneProperties, ApiError> {
    let client = CLIENT.get_or_init(init);

    let url = format!(
        "{}/call/Zone-House-{house_id}/getProperties",
        BASE_URL
    );

    let response = client.get(url).send().await?;

    let mut response_body = response.json::<ZoneProperties>().await?;
    response_body.heat_consumption =
        Some(parse_complex_str(&response_body.original_consumption.heat.clone().expect("Heat commodity not found"))?);

    Ok(response_body)
}

pub async fn get_thermostat_properties(house_id: u32) -> Result<ThermostatProperties, ApiError> {
    let client = CLIENT.get_or_init(init);

    let url = format!(
        "{}/call/Thermostat-House-{house_id}/getProperties",
        BASE_URL
    );

    let response = client.get(url).send().await?;

    let response_body = response.json::<ThermostatProperties>().await?;

    Ok(response_body)
}

const DELTA_TEMP: f64 = 1.0;

pub async fn set_target_temp(house_id: u32, temp: f64) -> Result<(), ApiError> {
    let client = CLIENT.get_or_init(init);

    let target_temp_min = temp - DELTA_TEMP;
    let target_temp_max = temp + DELTA_TEMP;

    if target_temp_min < 0.0 || target_temp_max > 35.0 {
        return Err(ApiError::DemkitError(format!("Invalid Temperature: {}. Temp must be between 0 and 35 degrees Celsius", target_temp_max)));
    }

    let url_min = format!(
        "{}/set/Thermostat-House-{house_id}/temperatureSetpointHeating/{target_temp_min}",
        BASE_URL
    );

    let url_min_away = format!(
        "{}/set/Thermostat-House-{house_id}/temperatureMin/{target_temp_min}",
        BASE_URL
    );

    let url_max = format!(
        "{}/set/Thermostat-House-{house_id}/temperatureSetpointCooling/{target_temp_max}",
        BASE_URL
    );

    let url_max_away = format!(
        "{}/set/Thermostat-House-{house_id}/temperatureMax/{target_temp_max}",
        BASE_URL
    );

    client.get(url_min).send().await?;
    client.get(url_min_away).send().await?;
    client.get(url_max).send().await?;
    client.get(url_max_away).send().await?;

    Ok(())
}