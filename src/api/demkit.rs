use std::{num::ParseFloatError, str::FromStr, sync::OnceLock};

use num_complex::{Complex, ParseComplexError};
use reqwest;
use serde::{Deserialize, Serialize};


pub mod battery;
pub mod meter;
pub mod solar;
pub mod thermal;
pub mod devices;
pub mod ha_entity;


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

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Commodity {
    #[serde(rename = "complex")]
    Complex(String),
    #[serde(rename = "real")]
    Real(f64),
}

#[derive(Deserialize, Debug)]
pub struct Commodities {
    #[serde(rename = "ELECTRICITY")]
    pub electricity: Option<Commodity>,
    #[serde(rename = "HEAT")]
    pub heat: Option<Commodity>,
}

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("Request failed")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Serde error")]
    SerdeError(#[from] serde_json::Error),
    #[error("Parse error")]
    ParseError(#[from] ParseComplexError<ParseFloatError>),
    #[error("DEMKIT API error")]
    DemkitError(String),
    
}

fn parse_complex_str(
    input: &Commodity,
) -> Result<Complex<f64>, ParseComplexError<ParseFloatError>> {
    match input {
        Commodity::Complex(input) => {
            Complex::from_str(&input[2..].replace("(", "").replace(")", ""))
        }
        Commodity::Real(input) => Ok(Complex::new(*input, 0.0)),
    }
}

