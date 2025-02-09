use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;

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
    let (house_id, battery_id) = id.into_inner();
    HttpResponse::Ok().body(format!("Battery {battery_id} in house {house_id}"))
}
