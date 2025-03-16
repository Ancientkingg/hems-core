use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;
use serde_json::json;

use crate::api::demkit;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/thermal/{id}")
            .service(get_by_id)
            .service(set_target_temp),
    );
}

#[derive(Serialize)]
struct ThermalInfo {
    current_temperature: f64,
    target_temperature: f64,
    heating_power: f64,
    consumption: f64,
}

#[get("")]
async fn get_by_id(id: web::Path<(u32, u32)>) -> impl Responder {
    let (_house_id, thermal_id) = id.into_inner();

    let zone_info = match demkit::thermal::get_current_zone_temp(thermal_id).await {
        Ok(properties) => properties,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    };

    let therm_info = match demkit::thermal::get_thermostat_properties(thermal_id).await {
        Ok(properties) => properties,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    };

    let target_temp = (therm_info.min_target_temp + therm_info.max_target_temp) / 2.0;

    let thermal_info = ThermalInfo {
        consumption: zone_info.heat_consumption.unwrap().norm(),
        current_temperature: zone_info.temperature,
        target_temperature: target_temp,
        heating_power: zone_info.valve_heat,
    };

    HttpResponse::Ok().json(thermal_info)
}

#[get("/target/{temp}")]
async fn set_target_temp(id: web::Path<(u32, u32, f64)>) -> impl Responder {
    let (house_id, _thermal_id, temp) = id.into_inner();
    match demkit::thermal::set_target_temp(house_id, temp).await {
        Ok(_) => {},
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    };

    HttpResponse::Ok().json(json!({"target_temperature": temp}))
}