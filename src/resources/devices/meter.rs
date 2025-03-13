use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;

use crate::api::demkit;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/meters/{id}")
            .service(get_by_id)
            .service(get_import)
            .service(get_export),
    );
}

#[derive(Serialize)]
struct MeterInfo {
    house_id: u32,
    meter_id: u32,
    total_import: f64,
    total_export: f64,
    current_import: Option<f64>,
    current_export: Option<f64>,
}

#[get("")]
async fn get_by_id(id: web::Path<(u32, u32)>) -> impl Responder {
    let (house_id, meter_id) = id.into_inner();

    let device_name = format!("SmartMeter-House-{}", house_id);

    let current_import = match demkit::meter::get_energy_import(house_id).await {
        Ok(measurement) => Some(measurement.value),
        Err(_) => None,
    };
    let current_export = match demkit::meter::get_energy_export(house_id).await {
        Ok(measurement) => Some(measurement.value),
        Err(_) => None,
    };

    let total_import = match demkit::devices::get_device_property(&device_name, "imported").await {
        Ok(value) => value,
        Err(_) => 0.0,
    };

    let total_export = match demkit::devices::get_device_property(&device_name, "exported").await {
        Ok(value) => value,
        Err(_) => 0.0,
    };

    let meter_info = MeterInfo {
        house_id,
        meter_id,
        current_import,
        current_export,
        total_import,
        total_export,
    };

    HttpResponse::Ok().json(meter_info)
}

#[get("/import")]
async fn get_import(id: web::Path<(u32, u32)>) -> impl Responder {
    let (house_id, _meter_id) = id.into_inner();

    let import = demkit::meter::get_energy_import(house_id).await;

    match import {
        Ok(measurement) => HttpResponse::Ok().json(measurement),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/export")]
async fn get_export(id: web::Path<(u32, u32)>) -> impl Responder {
    let (house_id, _meter_id) = id.into_inner();

    let import = demkit::meter::get_energy_export(house_id).await;

    match import {
        Ok(measurement) => HttpResponse::Ok().json(measurement),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
