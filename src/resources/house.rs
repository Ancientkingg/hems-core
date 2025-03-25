use actix_web::{get, web, HttpResponse, Responder};

#[path = "devices/devices.rs"]
mod devices;
use devices::{battery, meter, solar, thermal, timeshifters};

use crate::api::demkit;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/houses/{id}")
            .service(get_by_id)
            .service(get_time)
            .configure(battery::configure)
            .configure(meter::configure)
            .configure(solar::configure)
            .configure(thermal::configure)
            .configure(timeshifters::configure),
    );
}

#[get("")]
async fn get_by_id(path: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("House id: {}", path))
}

#[get("/time")]
async fn get_time() -> impl Responder {
    let current_time = demkit::get_time().await;
    HttpResponse::Ok().body(current_time.to_string())
}