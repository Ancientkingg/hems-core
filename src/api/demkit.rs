use std::{num::ParseFloatError, str::FromStr, sync::OnceLock};

use num_complex::{Complex, ParseComplexError};
use reqwest::{self, Error};
use serde::{Deserialize, Serialize};

const BASE_URL: &str = "http://localhost:5000";

static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

pub fn init() -> reqwest::Client {
    reqwest::Client::new()
}

#[derive(Serialize, Deserialize)]
pub struct Measurement {
    pub value: f64,
    pub unit: String,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum Commodity {
    complex(String),
    real(f64),
}

#[derive(Deserialize, Debug)]
struct Commodities {
    #[serde(rename = "ELECTRICITY")]
    pub electricity: Commodity,
}

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

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("Request failed")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Serde error")]
    SerdeError(#[from] serde_json::Error),
    #[error("Parse error")]
    ParseError(#[from] ParseComplexError<ParseFloatError>),
}

fn parse_complex_str(
    input: &Commodity,
) -> Result<Complex<f64>, ParseComplexError<ParseFloatError>> {
    match input {
        Commodity::complex(input) => {
            Complex::from_str(&input[2..].replace("(", "").replace(")", ""))
        }
        Commodity::real(input) => Ok(Complex::new(*input, 0.0)),
    }
}

pub async fn get_energy_import(house_id: u32) -> Result<Measurement, ApiError> {
    let client = CLIENT.get_or_init(init);

    let url = format!("{}/get/SmartMeter-House-{house_id}/consumption", BASE_URL);

    let response = client.get(url).send().await?;

    let response_body = response.json::<Commodities>().await?;

    let power = parse_complex_str(&response_body.electricity)?;

    Ok(Measurement {
        value: power.norm().max(0.0),
        unit: String::from("W"),
    })
}

pub async fn get_energy_export(house_id: u32) -> Result<Measurement, ApiError> {
    let client = CLIENT.get_or_init(init);

    let url = format!("{}/get/SmartMeter-House-{house_id}/consumption", BASE_URL);

    let response = client.get(url).send().await?;

    let response_body = response.json::<Commodities>().await?;

    let power = parse_complex_str(&response_body.electricity)?;

    Ok(Measurement {
        value: power.norm().min(0.0),
        unit: String::from("W"),
    })
}

pub async fn get_battery_properties(house_id: u32) -> Result<BatteryProperties, ApiError> {
    let client = CLIENT.get_or_init(init);

    let url = format!("{}/call/Battery-House-{house_id}/getProperties", BASE_URL);

    let response = client.get(url).send().await?;

    let mut response_body = response.json::<BatteryProperties>().await?;

    response_body.electricity_consumption =
        Some(parse_complex_str(&response_body.consumption.electricity)?);

    Ok(response_body)
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

pub async fn get_device_consumption(
    house_id: u32,
    device_name: &str,
) -> Result<Measurement, ApiError> {
    todo!("")
}

pub async fn get_device_property<T>(device_name: &str, property: &str) -> Result<T, ApiError>
where
    T: for<'a> Deserialize<'a>,
{
    let client = CLIENT.get_or_init(init);

    let url = format!("{}/get/{device_name}/{property}", BASE_URL);

    let response = client.get(url).send().await?;

    let response_body = response.json::<T>().await?;

    Ok(response_body)
}
