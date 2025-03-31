use std::env;

use super::{init, ApiError, EntityState, BASE_URL, CLIENT, LOAD_MAP, init_load_map};

pub async fn get_entity_consumption(entity_id: &str) -> Result<EntityState, ApiError> {
    let client = CLIENT.get_or_init(init);

    let load_map = LOAD_MAP.get_or_init(init_load_map).read().unwrap();

    let url = format!("{}/api/states/{}", BASE_URL, entity_id);

    let ha_token = env::var("HA_TOKEN").expect("HA_TOKEN must be set");

    let response = client.get(url).bearer_auth(ha_token).send().await?;

    if !response.status().is_success() {
        return Err(ApiError::HomeAssistantError("Failed to get device consumption".to_string()));
    }
    
    let response_body: EntityState = response.json::<EntityState>().await?;

    if load_map.contains_key(entity_id) {
        let mut consumption = "0".to_string();

        // consumption is renamed from state
        if response_body.consumption == "on" {
            consumption = load_map.get(entity_id).unwrap().to_string();
        }

        return Ok(EntityState {
            entity_id: entity_id.to_string(),
            consumption: consumption,
        });
    } else if response_body.consumption.parse::<f64>().is_err() {
        return Err(ApiError::LoadMapError("Device not found in load map".to_string()));
    }

    return Ok(EntityState {
        entity_id: entity_id.to_string(),
        consumption: response_body.consumption,
    });
}
