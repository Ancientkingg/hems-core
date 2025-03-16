use num_complex::Complex;
use serde::Deserialize;

use super::{init, parse_complex_str, ApiError, Commodities, BASE_URL, CLIENT};

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct BatteryProperties {
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
    pub soc: f64,
    #[serde(rename = "targetSoC")]
    pub target_soc: Option<f64>,
    pub cop: f64,
    pub capacity: f64,
    #[serde(rename = "chargingPowers")]
    pub charging_powers: Vec<f64>,
    #[serde(rename = "selfConsumption")]
    pub self_consumption: f64,
    #[serde(rename = "internalPowers")]
    pub internal_powers: Vec<f64>,
    #[serde(rename = "chargingEfficiency")]
    pub charging_efficiency: Vec<f64>,
    pub discrete: bool,
    #[serde(rename = "useInefficiency")]
    pub use_inefficiency: bool,

    pub electricity_consumption: Option<Complex<f64>>,
}

pub async fn get_battery_properties(house_id: u32) -> Result<BatteryProperties, ApiError> {
    let client = CLIENT.get_or_init(init);

    let url = format!("{}/call/Battery-House-{house_id}/getProperties", BASE_URL);

    let response = client.get(url).send().await?;

    let mut response_body = response.json::<BatteryProperties>().await?;

    response_body.electricity_consumption =
        Some(parse_complex_str(&response_body.consumption.electricity.clone().expect("Electricity commodity not found"))?);

    Ok(response_body)
}

pub async fn set_target_soc(house_id: u32, soc: Option<u32>) -> Result<BatteryProperties, ApiError> {
    let client = CLIENT.get_or_init(init);

    let soc = match soc {
        Some(soc) => soc.to_string(),
        None => "None".to_string(),
    };

    let url = format!("{}/set/Battery-House-{house_id}/targetSoC/{soc}", BASE_URL);

    let response = client.get(url).send().await?;

    let success = response.json::<bool>().await?;

    if success {
        get_battery_properties(house_id).await
    } else {
        Err(ApiError::DemkitError("Failed to set target SoC".to_string()))
    }

}