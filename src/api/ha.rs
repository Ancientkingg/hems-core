use std::sync::OnceLock;

use reqwest;
use serde::{Deserialize, Serialize};

const BASE_URL: &str = "http://localhost:8123";

static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

pub mod entity;

pub fn init() -> reqwest::Client {
    reqwest::Client::new()
}

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("Request failed")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Serde error")]
    SerdeError(#[from] serde_json::Error),
    #[error("Home Assistant API error")]
    HomeAssistantError(String),
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityState {
    pub entity_id: String,
    #[serde(rename(deserialize = "state"))]
    pub consumption: String,
}
