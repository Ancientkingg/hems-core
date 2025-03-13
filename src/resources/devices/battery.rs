use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;

use crate::api::demkit;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/battery/{id}").service(get_by_id));
}

#[derive(Serialize)]
enum BatteryStatus {
    Charging,
    Discharging,
    Idle,
}

#[derive(Serialize)]
struct BatteryInfo {
    capacity: f64,
    max_charge: f64,
    max_discharge: f64,
    state_of_charge: f64,
    status: BatteryStatus,
    consumption: f64,
}

#[get("")]
async fn get_by_id(id: web::Path<(u32, u32)>) -> impl Responder {
    let (house_id, _battery_id) = id.into_inner();

    let bp = match demkit::battery::get_battery_properties(house_id).await {
        Ok(properties) => properties,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    };

    let current_consumption = bp.electricity_consumption.unwrap().norm();

    let battery_status = if current_consumption > 1e2 {
        BatteryStatus::Discharging
    } else if current_consumption < -1e2 {
        BatteryStatus::Charging
    } else {
        BatteryStatus::Idle
    };

    let battery_info = BatteryInfo {
        capacity: bp.capacity,
        max_charge: *bp.charging_powers.last().unwrap_or(&0.0),
        max_discharge: -*bp.charging_powers.first().unwrap_or(&0.0),
        state_of_charge: bp.soc,
        status: battery_status,
        consumption: bp.electricity_consumption.unwrap().norm(),
    };

    HttpResponse::Ok().json(battery_info)
}
