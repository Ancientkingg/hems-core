use std::{num::ParseFloatError, str::FromStr, sync::OnceLock};

use num_complex::{Complex, ParseComplexError};
use once_cell::sync::Lazy;
use reqwest::{self, Error};
use serde::Deserialize;
use regex::Regex;

const BASE_URL: &str = "http://localhost:5000";

static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

pub fn init() -> reqwest::Client {
    reqwest::Client::new()
}

#[derive(Deserialize)]
pub struct Measurement {
    pub value: f64,
    pub unit: String,
}


#[derive(Deserialize)]
struct Commodities {
    #[serde(rename = "ELECTRICITY")]
    pub electricity: String,
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

// static COMPLEX_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"__\(([-+]?\d+\.?\d*)([-+]\d+\.?\d*)j\)").unwrap() );

fn parse_complex_str(input: &str) -> Result<Complex<f64>, ParseComplexError<ParseFloatError>> {
    Complex::from_str(&input[2..].replace("(", "").replace(")", ""))
}

pub async fn get_energy_import(house_id: u32) -> Result<Measurement, ApiError> {
    let client= CLIENT.get_or_init(init);

    let url = format!("{}/get/SmartMeter-House-{house_id}/consumption", BASE_URL);

    let response = client
        .get(url)
        .send()
        .await?;

    let response_body = response.json::<Commodities>().await?;

    let power = parse_complex_str(&response_body.electricity)?;

    Ok(Measurement { value: power.norm().max(0.0), unit: String::from("W") })
}

pub async fn get_energy_export(house_id: u32) -> Result<Measurement, ApiError> {
    let client= CLIENT.get_or_init(init);

    let url = format!("{}/get/SmartMeter-House-{house_id}/consumption", BASE_URL);

    let response = client
        .get(url)
        .send()
        .await?;

    let response_body = response.json::<Commodities>().await?;

    let power = parse_complex_str(&response_body.electricity)?;

    Ok(Measurement { value: power.norm().min(0.0), unit: String::from("W") })
}

pub async fn get_battery_status(house_id: u32) -> Result<Measurement, ApiError> {
    todo!("")
}

pub async fn get_device_consumption(house_id: u32, device_name: &str) -> Result<Measurement, ApiError> {
    todo!("")
}