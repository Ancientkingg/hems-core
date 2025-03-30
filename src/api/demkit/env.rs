use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{init, ApiError, BASE_URL, CLIENT};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SimConfig {
    pub time_base: u64,
    pub time_offset: i64,
    pub time_zone: String,
    pub intervals: u64,
    pub start_time: u64,
    pub database: String,
    pub data_prefix: String,
    #[serde(rename = "clearDB")]
    pub clear_db: bool,
    pub extended_logging: bool,
    pub log_devices: bool,
    pub log_flow: bool,
    pub enable_persistence: bool,

    pub weather_file: String,
    pub irradiance_file: String,
    pub ventilation_file: String,
    pub gain_file: String,
    pub dhw_file: String,

    pub house_num: u32,
    pub use_islanding: bool,

    pub photo_voltaic_settings: String,
    pub battery_settings: String,
    pub heating_settings: String,

    pub use_fill_method: bool,
    #[serde(rename = "usePP")]
    pub use_pp: bool,
    pub ctrl_time_base: u64,

    pub thermostat_start_times: String,
    pub thermostat_setpoints: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HostEntityParams {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherEntityParams {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SunEntityParams {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EntityParams {
    Host(HostEntityParams),
    Weather(WeatherEntityParams),
    Sun(SunEntityParams),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entity {
    pub r#type: String,
    pub entity: Value,
}

impl Entity {
    pub fn new(entity_params: EntityParams) -> Self {
        let (entity_type, entity)  = match entity_params {
            EntityParams::Host(params) => ("host", serde_json::to_value(params).unwrap()),
            EntityParams::Weather(params) => ("weather", serde_json::to_value(params).unwrap()),
            EntityParams::Sun(params) => ("sun", serde_json::to_value(params).unwrap()),
        };

        Self {
            r#type: entity_type.to_string(),
            entity,
        }
    }
}

pub async fn add_host() -> Result<(), ApiError> {
    let client = CLIENT.get_or_init(init);

    let url = format!("{}/composer/entities", BASE_URL);

    let inner = HostEntityParams {
        name: "Host".to_string(),
    };
    let entity = Entity::new(EntityParams::Host(inner));

    let response = client.post(url).json(&entity).send().await?;

    if response.status().is_success() {
        Ok(())
    } else {
        let error_message = response.text().await?;
        Err(ApiError::DemkitError(format!(
            "Failed to set host: {}",
            error_message
        )))
    }
}

pub async fn add_weather() -> Result<(), ApiError> {
    let client = CLIENT.get_or_init(init);

    let url = format!("{}/composer/entities", BASE_URL);

    let inner = WeatherEntityParams {
        name: "Weather".to_string(),
    };
    let entity = Entity::new(EntityParams::Weather(inner));

    let response = client.post(url).json(&entity).send().await?;

    if response.status().is_success() {
        Ok(())
    } else {
        let error_message = response.text().await?;
        Err(ApiError::DemkitError(format!(
            "Failed to add weather: {}",
            error_message
        )))
    }
}

pub async fn add_sun() -> Result<(), ApiError> {
    let client = CLIENT.get_or_init(init);

    let url = format!("{}/composer/entities", BASE_URL);

    let inner = SunEntityParams {
        name: "Sun".to_string(),
    };
    let entity = Entity::new(EntityParams::Sun(inner));

    let response = client.post(url).json(&entity).send().await?;

    if response.status().is_success() {
        Ok(())
    } else {
        let error_message = response.text().await?;
        Err(ApiError::DemkitError(format!(
            "Failed to add sun: {}",
            error_message
        )))
    }
}

pub async fn set_config(config: SimConfig) -> Result<(), ApiError> {
    let client = CLIENT.get_or_init(init);

    let url = format!("{}/composer/config", BASE_URL);

    let response = client.post(url).json(&config).send().await?;

    if response.status().is_success() {
        Ok(())
    } else {
        let error_message = response.text().await?;
        Err(ApiError::DemkitError(format!(
            "Failed to set config: {}",
            error_message
        )))
    }
}

pub async fn load() -> Result<(), ApiError> {
    let client = CLIENT.get_or_init(init);

    let url = format!("{}/composer/load", BASE_URL);

    let response = client.post(url).send().await?;

    if response.status().is_success() {
        Ok(())
    } else {
        let error_message = response.text().await?;
        Err(ApiError::DemkitError(format!(
            "Failed to load: {}",
            error_message
        )))
    }
}

pub async fn start() -> Result<(), ApiError> {
    let client = CLIENT.get_or_init(init);

    let url = format!("{}/composer/start", BASE_URL);

    let response = client.post(url).send().await?;

    if response.status().is_success() {
        Ok(())
    } else {
        let error_message = response.text().await?;
        Err(ApiError::DemkitError(format!(
            "Failed to start: {}",
            error_message
        )))
    }
}