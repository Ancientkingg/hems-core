use serde_json::json;

use super::{ApiError, BASE_URL, CLIENT};

pub async fn add_entity(entity_id: &str) -> Result<(), ApiError> {
    let client = CLIENT.get_or_init(super::init);

    let url = format!("{}/entity", BASE_URL);

    let request_json = json!({"entity_id": entity_id});

    let response: reqwest::Response = client.post(url).json(&request_json).send().await?;

    if !response.status().is_success() {
        return Err(ApiError::DemkitError("Failed to add device".to_string()));
    }

    return Ok(());
}
