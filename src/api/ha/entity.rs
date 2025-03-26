use std::env;

use super::{init, ApiError, EntityState, BASE_URL, CLIENT};

pub async fn get_entity_consumption(entity_id: &str) -> Result<EntityState, ApiError> {
    let client = CLIENT.get_or_init(init);

    let url = format!("{}/api/states/{}", BASE_URL, entity_id);

    let ha_token = env::var("HA_TOKEN").expect("HA_TOKEN must be set");

    println!("{}", url);

    let response = client.get(url).bearer_auth(ha_token).send().await?;

    let response_body = response.json::<EntityState>().await?;

    println!("{}: {}", response_body.entity_id, response_body.consumption);

    return Ok(EntityState {
        entity_id: entity_id.to_string(),
        consumption: response_body.consumption,
    });
}
