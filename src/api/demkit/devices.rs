use serde::Deserialize;

use super::{init, parse_complex_str, ApiError, Commodities, Measurement, BASE_URL, CLIENT};

pub async fn get_device_consumption(
    _house_id: u32,
    device_name: &str,
) -> Result<Commodities, ApiError> {
    let consumption = get_device_property::<Commodities>(device_name, "consumption").await?;

    Ok(consumption)
}

pub async fn get_device_electricity_consumption(
    house_id: u32,
    device_name: &str,
) -> Result<Measurement, ApiError> {
    let consumption = get_device_consumption(house_id, device_name).await?;

    match consumption.electricity {
        Some(power) => {
            let power = parse_complex_str(&power)?;
            let cons = power.norm() * power.re.signum();

            Ok(Measurement {
                value: cons.max(0.0),
                unit: String::from("W"),
            })
        }
        None => Ok(Measurement {
            value: 0.0,
            unit: String::from("W"),
        }),
    }
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