use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{init, ApiError, BASE_URL, CLIENT};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Time {
    pub time: u64,
}

pub async fn pause_simulation() -> Result<(), ApiError> {
    let client = CLIENT.get_or_init(init);

    let url = format!("{}/simulation/pause", *BASE_URL);
    let response = client.post(&url).send().await?;

    if response.status().is_success() {
        Ok(())
    } else {
        let error_message = response.text().await?;
        Err(ApiError::DemkitError(format!(
            "Failed to pause simulation: {}",
            error_message
        )))
    }
}

pub async fn resume_simulation() -> Result<(), ApiError> {
    let client = CLIENT.get_or_init(init);

    let url = format!("{}/simulation/resume", *BASE_URL);
    let response = client.post(&url).send().await?;

    if response.status().is_success() {
        Ok(())
    } else {
        let error_message = response.text().await?;
        Err(ApiError::DemkitError(format!(
            "Failed to resume simulation: {}",
            error_message
        )))
    }
}

pub async fn stop_simulation() -> Result<(), ApiError> {
    let client = CLIENT.get_or_init(init);

    let url = format!("{}/simulation/stop", *BASE_URL);
    let response = client.post(&url).send().await?;

    if response.status().is_success() {
        Ok(())
    } else {
        let error_message = response.text().await?;
        Err(ApiError::DemkitError(format!(
            "Failed to stop simulation: {}",
            error_message
        )))
    }
}

pub async fn set_time(time: Time) -> Result<(), ApiError> {
    let client = CLIENT.get_or_init(init);

    let url = format!("{}/time", *BASE_URL);
    let response = client.post(&url).json(&time).send().await?;

    if response.status().is_success() {
        Ok(())
    } else {
        let error_message = response.text().await?;
        Err(ApiError::DemkitError(format!(
            "Failed to set time: {}",
            error_message
        )))
    }
}