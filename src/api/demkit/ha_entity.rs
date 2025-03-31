use serde_json::json;

use super::{ApiError, BASE_URL, CLIENT};
use crate::api::ha::{LOAD_MAP, init_load_map};

#[derive(serde::Deserialize)]
pub struct EntityRequest {
    entity_id: String,
    consumption: String,
}

pub async fn add_entity(entity: EntityRequest) -> Result<(), ApiError> {
    let client = CLIENT.get_or_init(super::init);
    let mut load_map = LOAD_MAP.get_or_init(init_load_map).write().unwrap();

    let url = format!("{}/entity", BASE_URL);

    let request_json = json!({"entity_id": entity.entity_id});

    let response = client.post(url).json(&request_json).send().await?;

    if !response.status().is_success() {
        return Err(ApiError::DemkitError("Failed to add device".to_string()));
    }

    if entity.consumption != "-1" {
        load_map.insert(entity.entity_id.to_string(), entity.consumption);
    }

    return Ok(());
}
