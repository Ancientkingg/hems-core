use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_actix_web::scope;

use crate::api::demkit;

pub fn configure(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(
        scope::scope("/meters/{id}")
            .service(get_by_id)
            .service(get_import)
            .service(get_export),
    );
}

#[derive(Serialize, ToSchema)]
struct MeterInfo {
    /// House ID
    house_id: u32,
    /// Meter ID
    meter_id: u32,
    /// Total energy imported
    total_import: f64,
    /// Total energy exported
    total_export: f64,
    /// Current energy import
    current_import: Option<f64>,
    /// Current energy export
    current_export: Option<f64>,
}

#[utoipa::path(
    get,
    tag = "Meter",
    description = "Get properties of a meter device",
    responses(
        (status = 200, description = "Get meter information", body = MeterInfo),
        (status = 500, description = "Failed to get meter information"),
    ),
)]
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

#[utoipa::path(
    get,
    tag = "Meter",
    description = "Get current energy import",
    responses(
        (status = 200, description = "Get energy import", body = f64),
        (status = 500, description = "Failed to get energy import"),
    ),
)]
#[get("/import")]
async fn get_import(id: web::Path<(u32, u32)>) -> impl Responder {
    let (house_id, _meter_id) = id.into_inner();

    let import = demkit::meter::get_energy_import(house_id).await;

    match import {
        Ok(measurement) => HttpResponse::Ok().json(measurement),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[utoipa::path(
    get,
    tag = "Meter",
    description = "Get current energy export",
    responses(
        (status = 200, description = "Get energy export", body = f64),
        (status = 500, description = "Failed to get energy export"),
    ),
)]
#[get("/export")]
async fn get_export(id: web::Path<(u32, u32)>) -> impl Responder {
    let (house_id, _meter_id) = id.into_inner();

    let import = demkit::meter::get_energy_export(house_id).await;

    match import {
        Ok(measurement) => HttpResponse::Ok().json(measurement),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
