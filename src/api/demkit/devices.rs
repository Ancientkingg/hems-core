use serde::Deserialize;

use super::{init, ApiError, Measurement, BASE_URL, CLIENT};

pub async fn get_device_consumption(
    _house_id: u32,
    _device_name: &str,
) -> Result<Measurement, ApiError> {
    todo!("")
}

pub async fn get_device_property<T>(device_name: &str, property: &str) -> Result<T, ApiError>
where
    T: for<'a> Deserialize<'a>,
{
    let client = CLIENT.get_or_init(init);

    let url = format!("{}/get/{device_name}/{property}", BASE_URL);

    let response = client.get(url).send().await?;

    let response_body = response.json::<T>().await?;

    Ok(response_body)
}