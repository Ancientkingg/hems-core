use num_complex::Complex;
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
#[serde(rename_all = "camelCase")]
pub struct TimeShifterEntityParams {
    pub name: String,
    pub profile: Vec<Complex<f64>>,
    pub time_base: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BatteryEntityParams {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SolarEntityParams {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CurtEntityParams {
    pub name: String,
    pub filename: String,
    pub filename_reactive: String,
    pub column: u64,
    pub time_base: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ZoneEntityParams {
    pub name: String,
    pub r_floor: f64,
    pub r_envelope: f64,
    pub c_floor: f64,
    pub c_zone: f64,
    pub initial_temperature: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MeterEntityParams {
    pub name: String,
    pub commodities: Vec<String>,
    pub weights: Vec<(String, f64)>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ThermostatEntityParams {
    pub name: String,
    pub temperature_setpoint_heating: f64,
    pub temperature_setpoint_cooling: f64,
    pub temperature_min: f64,
    pub temperature_max: f64,
    pub temperature_deadband: Vec<f64>,
    pub preheating_time: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DhwEntityParams {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HeatSourceEntityParams {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HeatPumpEntityParams {
    pub name: String,
    pub producing_temperatures: Vec<f64>,
    pub producing_powers: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EntityParams {
    Host(HostEntityParams),
    Weather(WeatherEntityParams),
    Sun(SunEntityParams),
    TimeShifter(TimeShifterEntityParams),
    Battery(BatteryEntityParams),
    Solar(SolarEntityParams),
    Curt(CurtEntityParams),
    Zone(ZoneEntityParams),
    Meter(MeterEntityParams),
    Thermostat(ThermostatEntityParams),
    Dhw(DhwEntityParams),
    HeatSource(HeatSourceEntityParams),
    HeatPump(HeatPumpEntityParams),
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
            EntityParams::TimeShifter(params) => ("timeshiftable", serde_json::to_value(params).unwrap()),
            EntityParams::Battery(params) => ("battery", serde_json::to_value(params).unwrap()),
            EntityParams::Solar(params) => ("solar_panel", serde_json::to_value(params).unwrap()),
            EntityParams::Curt(params) => ("curt", serde_json::to_value(params).unwrap()),
            EntityParams::Zone(params) => ("zone", serde_json::to_value(params).unwrap()),
            EntityParams::Meter(params) => ("meter", serde_json::to_value(params).unwrap()),
            EntityParams::Thermostat(params) => ("thermostat", serde_json::to_value(params).unwrap()),
            EntityParams::Dhw(params) => ("dhw", serde_json::to_value(params).unwrap()),
            EntityParams::HeatSource(params) => ("heat_source", serde_json::to_value(params).unwrap()),
            EntityParams::HeatPump(params) => ("heat_pump", serde_json::to_value(params).unwrap()),
        };

        Self {
            r#type: entity_type.to_string(),
            entity,
        }
    }
}

pub async fn add_host(house_id: u32) -> Result<(), ApiError> {
    let client = CLIENT.get_or_init(init);

    let url = format!("{}/composer/entities", BASE_URL);

    let inner = HostEntityParams {
        name: format!("House-{house_id}"),
    };
    let entity = Entity::new(EntityParams::Host(inner));

    let response = client.put(url).json(&entity).send().await?;

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

pub async fn add_weather(house_id: u32) -> Result<(), ApiError> {
    let client = CLIENT.get_or_init(init);

    let url = format!("{}/composer/entities", BASE_URL);

    let inner = WeatherEntityParams {
        name: format!("Weather-House-{house_id}"),
    };
    let entity = Entity::new(EntityParams::Weather(inner));

    let response = client.put(url).json(&entity).send().await?;

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

pub async fn add_sun(house_id: u32) -> Result<(), ApiError> {
    let client = CLIENT.get_or_init(init);

    let url = format!("{}/composer/entities", BASE_URL);

    let inner = SunEntityParams {
        name: format!("Sun-House-{house_id}"),
    };
    let entity = Entity::new(EntityParams::Sun(inner));

    let response = client.put(url).json(&entity).send().await?;

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

pub async fn add_timeshifter(house_id: u32, mut inner: TimeShifterEntityParams) -> Result<(), ApiError> {
    let client = CLIENT.get_or_init(init);

    let url = format!("{}/composer/entities", BASE_URL);

    let name = inner.name.clone();
    inner.name = format!("{name}-House-{house_id}");

    let entity = Entity::new(EntityParams::TimeShifter(inner));

    let response = client.put(url).json(&entity).send().await?;

    if response.status().is_success() {
        Ok(())
    } else {
        let error_message = response.text().await?;
        Err(ApiError::DemkitError(format!(
            "Failed to set timeshifter: {}",
            error_message
        )))
    }
}

pub async fn add_battery(house_id: u32) -> Result<(), ApiError> {
    let client = CLIENT.get_or_init(init);

    let url = format!("{}/composer/entities", BASE_URL);

    let inner = BatteryEntityParams {
        name: format!("Battery-House-{house_id}"),
    };
    let entity = Entity::new(EntityParams::Battery(inner));

    let response = client.put(url).json(&entity).send().await?;

    if response.status().is_success() {
        Ok(())
    } else {
        let error_message = response.text().await?;
        Err(ApiError::DemkitError(format!(
            "Failed to set battery: {}",
            error_message
        )))
    }
}

pub async fn add_solar(house_id: u32) -> Result<(), ApiError> {
    let client = CLIENT.get_or_init(init);

    let url = format!("{}/composer/entities", BASE_URL);

    let inner = SolarEntityParams {
        name: format!("PV-House-{house_id}"),
    };
    let entity = Entity::new(EntityParams::Solar(inner));

    let response = client.put(url).json(&entity).send().await?;

    if response.status().is_success() {
        Ok(())
    } else {
        let error_message = response.text().await?;
        Err(ApiError::DemkitError(format!(
            "Failed to set solar: {}",
            error_message
        )))
    }
}

pub async fn add_curt(house_id: u32, mut inner: CurtEntityParams) -> Result<(), ApiError> {
    let client = CLIENT.get_or_init(init);

    let url = format!("{}/composer/entities", BASE_URL);

    let name = inner.name.clone();
    inner.name = format!("{name}-House-{house_id}");

    let entity = Entity::new(EntityParams::Curt(inner));

    let response = client.put(url).json(&entity).send().await?;

    if response.status().is_success() {
        Ok(())
    } else {
        let error_message = response.text().await?;
        Err(ApiError::DemkitError(format!(
            "Failed to set curt: {}",
            error_message
        )))
    }
}

pub async fn add_zone(house_id: u32, mut inner: ZoneEntityParams) -> Result<(), ApiError> {
    let client = CLIENT.get_or_init(init);

    let url = format!("{}/composer/entities", BASE_URL);

    let name = inner.name.clone();
    inner.name = format!("{name}-House-{house_id}");

    let entity = Entity::new(EntityParams::Zone(inner));

    let response = client.put(url).json(&entity).send().await?;

    if response.status().is_success() {
        Ok(())
    } else {
        let error_message = response.text().await?;
        Err(ApiError::DemkitError(format!(
            "Failed to set zone: {}",
            error_message
        )))
    }
}

pub async fn add_meter(house_id: u32, mut inner: MeterEntityParams) -> Result<(), ApiError> {
    let client = CLIENT.get_or_init(init);

    let url = format!("{}/composer/entities", BASE_URL);

    let name = inner.name.clone();
    inner.name = format!("{name}-House-{house_id}");

    let entity = Entity::new(EntityParams::Meter(inner));

    let response = client.put(url).json(&entity).send().await?;

    if response.status().is_success() {
        Ok(())
    } else {
        let error_message = response.text().await?;
        Err(ApiError::DemkitError(format!(
            "Failed to set meter: {}",
            error_message
        )))
    }
}

pub async fn add_thermostat(house_id: u32, mut inner: ThermostatEntityParams) -> Result<(), ApiError> {
    let client = CLIENT.get_or_init(init);

    let url = format!("{}/composer/entities", BASE_URL);

    let name = inner.name.clone();
    inner.name = format!("{name}-House-{house_id}");

    let entity = Entity::new(EntityParams::Thermostat(inner));

    let response = client.put(url).json(&entity).send().await?;

    if response.status().is_success() {
        Ok(())
    } else {
        let error_message = response.text().await?;
        Err(ApiError::DemkitError(format!(
            "Failed to set thermostat: {}",
            error_message
        )))
    }
}

pub async fn add_dhw(house_id: u32, mut inner: DhwEntityParams) -> Result<(), ApiError> {
    let client = CLIENT.get_or_init(init);

    let url = format!("{}/composer/entities", BASE_URL);

    let name = inner.name.clone();
    inner.name = format!("{name}-House-{house_id}");

    let entity = Entity::new(EntityParams::Dhw(inner));

    let response = client.put(url).json(&entity).send().await?;

    if response.status().is_success() {
        Ok(())
    } else {
        let error_message = response.text().await?;
        Err(ApiError::DemkitError(format!(
            "Failed to set dhw: {}",
            error_message
        )))
    }
}

pub async fn add_heat_source(house_id: u32, mut inner: HeatSourceEntityParams) -> Result<(), ApiError> {
    let client = CLIENT.get_or_init(init);

    let url = format!("{}/composer/entities", BASE_URL);

    let name = inner.name.clone();
    inner.name = format!("{name}-House-{house_id}");

    let entity = Entity::new(EntityParams::HeatSource(inner));

    let response = client.put(url).json(&entity).send().await?;

    if response.status().is_success() {
        Ok(())
    } else {
        let error_message = response.text().await?;
        Err(ApiError::DemkitError(format!(
            "Failed to set heat source: {}",
            error_message
        )))
    }
}

pub async fn add_heat_pump(house_id: u32, mut inner: HeatPumpEntityParams) -> Result<(), ApiError> {
    let client = CLIENT.get_or_init(init);

    let url = format!("{}/composer/entities", BASE_URL);

    let name = inner.name.clone();
    inner.name = format!("{name}-House-{house_id}");

    let entity = Entity::new(EntityParams::HeatPump(inner));

    let response = client.put(url).json(&entity).send().await?;

    if response.status().is_success() {
        Ok(())
    } else {
        let error_message = response.text().await?;
        Err(ApiError::DemkitError(format!(
            "Failed to set heat pump: {}",
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

pub async fn reset() -> Result<(), ApiError> {
    let client = CLIENT.get_or_init(init);

    let url = format!("{}/composer/reset", BASE_URL);

    let response = client.post(url).send().await?;

    if response.status().is_success() {
        Ok(())
    } else {
        let error_message = response.text().await?;
        Err(ApiError::DemkitError(format!(
            "Failed to reset: {}",
            error_message
        )))
    }
}

pub async fn remove_entity(house_id: u32, name: &str) -> Result<(), ApiError> {
    let client = CLIENT.get_or_init(init);

    let url = format!("{}/composer/entities/{name}-House-{house_id}", BASE_URL);

    let response = client.delete(url).send().await?;

    if response.status().is_success() {
        Ok(())
    } else {
        let error_message = response.text().await?;
        Err(ApiError::DemkitError(format!(
            "Failed to remove timeshifter: {}",
            error_message
        )))
    }
}