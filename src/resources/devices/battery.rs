use actix_web::{delete, get, post, web, HttpResponse, Responder};
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_actix_web::scope;

use crate::api::demkit::{self, battery::BatteryProperties, env::BatteryEntityParams};

pub fn configure(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(
        scope::scope("/battery/{id}")
            .service(get_by_id)
            .service(add_by_id)
            .service(remove_by_id)
            .service(set_target_soc)
            .service(set_target_soc_none),
    );
}

#[derive(Serialize, ToSchema)]
enum BatteryStatus {
    Charging,
    Discharging,
    Idle,
}

#[derive(Serialize, ToSchema)]
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

#[utoipa::path(
    get,
    path = "/houses/{house_id}/battery/{battery_id}",
    responses(
        (status = 200, description = "Get battery properties", body = BatteryInfo),
        (status = 400, description = "Invalid battery ID"),
        (status = 500, description = "Internal server error"),
    ),
    params(
        ("house_id" = u32, description = "House ID"),
        ("battery_id" = u32, description = "Battery ID"),
    )
)]
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

#[utoipa::path(
    post,
    path = "/houses/{house_id}/battery/{entity_name}",
    responses(
        (status = 200, description = "Battery added successfully"),
        (status = 500, description = "Error adding battery"),
    ),
    params(
        ("house_id" = u32, description = "House ID"),
        ("entity_name" = String, description = "Name of the battery entity"),
    )
)]
#[post("")]
async fn add_by_id(
    id: web::Path<(u32, String)>,
    _params: web::Json<BatteryEntityParams>,
) -> impl Responder {
    let (house_id, entity_name) = id.into_inner();

    match demkit::env::add_battery(house_id).await {
        Ok(_) => HttpResponse::Ok().body(format!("{entity_name} added successfully")),
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    };

    HttpResponse::Ok().body(format!("{entity_name} added successfully"))
}

#[utoipa::path(
    delete,
    path = "/houses/{house_id}/battery/{entity_name}",
    responses(
        (status = 200, description = "Battery removed successfully"),
        (status = 500, description = "Error removing battery"),
    ),
    params(
        ("house_id" = u32, description = "House ID"),
        ("entity_name" = String, description = "Name of the battery entity"),
    )
)]
#[delete("")]
async fn remove_by_id(id: web::Path<(u32, String)>) -> impl Responder {
    let (house_id, entity_name) = id.into_inner();

    match demkit::env::remove_entity(house_id, entity_name.as_str()).await {
        Ok(_) => HttpResponse::Ok().body(format!("{entity_name} removed successfully")),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    }
}

#[utoipa::path(
    get,
    path = "/houses/{house_id}/battery/{battery_id}/target/{soc}",
    responses(
        (status = 200, description = "Set target SOC", body = BatteryInfo),
        (status = 500, description = "Error setting target SOC"),
    ),
    params(
        ("house_id" = u32, description = "House ID"),
        ("battery_id" = u32, description = "Battery ID"),
        ("soc" = u32, description = "Target state of charge"),
    )
)]
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

#[utoipa::path(
    get,
    path = "/houses/{house_id}/battery/{battery_id}/target",
    responses(
        (status = 200, description = "Unset target SOC", body = BatteryInfo),
        (status = 500, description = "Error unsetting target SOC"),
    ),
    params(
        ("house_id" = u32, description = "House ID"),
        ("battery_id" = u32, description = "Battery ID"),
    )
)]
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
