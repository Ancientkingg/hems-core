use super::{init, parse_complex_str, ApiError, Commodities, Measurement, BASE_URL, CLIENT};

pub async fn get_energy_import(house_id: u32) -> Result<Measurement, ApiError> {
    let client = CLIENT.get_or_init(init);

    let url = format!("{}/get/SmartMeter-House-{house_id}/consumption", BASE_URL);

    let response = client.get(url).send().await?;

    let response_body = response.json::<Commodities>().await?;

    let power = parse_complex_str(&response_body.electricity.expect("Electricity commodity not found"))?;

    let cons = power.norm() * power.re.signum();

    Ok(Measurement {
        value: cons.max(0.0),
        unit: String::from("W"),
    })
}

pub async fn get_energy_export(house_id: u32) -> Result<Measurement, ApiError> {
    let client = CLIENT.get_or_init(init);

    let url = format!("{}/get/SmartMeter-House-{house_id}/consumption", BASE_URL);

    let response = client.get(url).send().await?;

    let response_body = response.json::<Commodities>().await?;

    let power = parse_complex_str(&response_body.electricity.expect("Electricity commodity not found"))?;

    let cons = -power.norm() * power.re.signum();

    Ok(Measurement {
        value: cons.max(0.0),
        unit: String::from("W"),
    })
}

