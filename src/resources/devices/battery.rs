use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Serialize;

use crate::api::demkit::{self, battery::BatteryProperties};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/battery/{id}")
            .service(get_by_id)
            .service(set_target_soc)
            .service(set_target_soc_none),
    );
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
    target_soc: Option<f64>,
    status: BatteryStatus,
    consumption: f64,
}

impl From<BatteryProperties> for BatteryInfo {
    fn from(bp: BatteryProperties) -> Self {
        let elec = bp.electricity_consumption.unwrap();
        let current_consumption = elec.norm() * elec.re.signum();

        let battery_status = if current_consumption > 1e2 {
            BatteryStatus::Charging
        } else if current_consumption < -1e2 {
            BatteryStatus::Discharging
        } else {
            BatteryStatus::Idle
        };

        let battery_info = BatteryInfo {
            capacity: bp.capacity,
            max_charge: *bp.charging_powers.last().unwrap_or(&0.0),
            max_discharge: -*bp.charging_powers.first().unwrap_or(&0.0),
            state_of_charge: bp.soc,
            target_soc: bp.target_soc,
            status: battery_status,
            consumption: bp.electricity_consumption.unwrap().norm(),
        };

        battery_info
    }
}

#[get("")]
async fn get_by_id(id: web::Path<(u32, u32)>) -> impl Responder {
    let (house_id, _battery_id) = id.into_inner();

    let bp = match demkit::battery::get_battery_properties(house_id).await {
        Ok(properties) => properties,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    };

    let battery_info = BatteryInfo::from(bp);

    HttpResponse::Ok().json(battery_info)
}

#[get("/target/{soc}")]
async fn set_target_soc(id: web::Path<(u32, u32, u32)>) -> impl Responder {
    let (house_id, _battery_id, target_soc) = id.into_inner();

    let bp = match demkit::battery::set_target_soc(house_id, Some(target_soc)).await {
        Ok(properties) => properties,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    };

    let battery_info = BatteryInfo::from(bp);

    HttpResponse::Ok().json(battery_info)
}

#[get("/target")]
async fn set_target_soc_none(id: web::Path<(u32, u32)>) -> impl Responder {
    let (house_id, _battery_id) = id.into_inner();

    let bp = match demkit::battery::set_target_soc(house_id, None).await {
        Ok(properties) => properties,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    };

    let battery_info = BatteryInfo::from(bp);

    HttpResponse::Ok().json(battery_info)
}
