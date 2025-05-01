use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;
use serde_json::json;
use utoipa::ToSchema;
use utoipa_actix_web::scope;

use crate::api::demkit;

pub fn configure(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(
        scope::scope("/thermal/{id}")
            .service(get_by_id)
            .service(set_target_temp),
    );
}

#[derive(Serialize, ToSchema)]
struct ThermalInfo {
    /// Current temperature in the zone in Celsius
    current_temperature: f64,
    /// Target temperature set for the zone in Celsius
    target_temperature: f64,
    /// Heating power
    heating_power: f64,
    /// Heat consumption
    consumption: f64,
}

#[utoipa::path(
    get,
    tag = "Thermal",
    description = "Get properties of a thermal device",
    responses(
        (status = 200, description = "Get thermal information", body = ThermalInfo),
        (status = 500, description = "Failed to get thermal information"),
    ),
    params(
        ("house_id" = u32, description = "House ID"),
        ("thermal_id" = u32, description = "Thermal ID"),
    ),
)]
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

#[utoipa::path(
    post,
    tag = "Thermal",
    description = "Set target temperature for a thermal device",
    responses(
        (status = 200, description = "Set target temperature successfully"),
        (status = 500, description = "Failed to set target temperature"),
    ),
    params(
        ("house_id" = u32, description = "House ID"),
        ("thermal_id" = u32, description = "Thermal ID"),
        ("temp" = f64, description = "Target temperature"),
    ),
)]
#[get("/target/{temp}")]
async fn set_target_temp(id: web::Path<(u32, u32, f64)>) -> impl Responder {
    let (house_id, _thermal_id, temp) = id.into_inner();
    match demkit::thermal::set_target_temp(house_id, temp).await {
        Ok(_) => {},
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    };

    HttpResponse::Ok().json(json!({"target_temperature": temp}))
}