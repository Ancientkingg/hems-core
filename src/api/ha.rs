use std::{collections::HashMap, sync::{OnceLock, RwLock}};

use reqwest;
use serde::{Deserialize, Serialize};

const BASE_URL: &str = "http://localhost:8123";

static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

pub static LOAD_MAP: OnceLock<RwLock<HashMap<String, String>>> = OnceLock::new();

pub mod entity;

pub fn init() -> reqwest::Client {
    reqwest::Client::new()
}

pub fn init_load_map() -> RwLock<HashMap<String, String>> {
    RwLock::new(HashMap::new())
}

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("Request failed: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Serde error; {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("Home Assistant API error: {0}")]
    HomeAssistantError(String),
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityState {
    pub entity_id: String,
    #[serde(rename(deserialize = "state"))]
    pub consumption: String,
}
