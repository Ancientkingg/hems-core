use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;

use crate::api::demkit;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/solar/{id}")
            .service(get_by_id)
    );
}

#[derive(Serialize)]
struct SolarInfo {
    consumption: f64
}

#[get("")]
async fn get_by_id(id: web::Path<(u32, u32)>) -> impl Responder {
    let (house_id, solar_id) = id.into_inner();
    HttpResponse::Ok().body(format!("Solar panel {solar_id} in house {house_id}"))
}