use actix_web::{delete, get, post, web, HttpResponse, Responder};
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_actix_web::scope;

use crate::api::demkit::{self, env::SolarEntityParams};

pub fn configure(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(
        scope::scope("/solar/{id}")
            .service(get_by_id)
            .service(add_by_id)
            .service(remove_by_id)
            .service(toggle),
    );
}

#[derive(Serialize, ToSchema)]
struct SolarInfo {
    /// Current electricity consumption (can be negative to indicate generation)
    consumption: f64
}

#[utoipa::path(
    get,
    tag = "Solar",
    description = "Get solar information.",
    responses(
        (status = 200, description = "Get solar information", body = SolarInfo),
        (status = 500, description = "Failed to get solar information"),
    ),
    params(
        ("house_id" = u32, description = "House ID"),
        ("solar_id" = u32, description = "Solar ID"),
    )
)]
#[get("")]
async fn get_by_id(id: web::Path<(u32, u32)>) -> impl Responder {
    let (_house_id, solar_id) = id.into_inner();

    let sp = match demkit::solar::get_solar_properties(solar_id).await {
        Ok(properties) => properties,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    };

    let solar_info = SolarInfo {
        consumption: sp.electricity_consumption.unwrap().norm(),
    };

    HttpResponse::Ok().json(solar_info)
}

#[utoipa::path(
    post,
    tag = "Solar",
    description = "Add a solar entity.",
    request_body = SolarEntityParams,
    responses(
        (status = 200, description = "Add solar entity successfully"),
        (status = 500, description = "Failed to add solar entity"),
    ),
    params(
        ("house_id" = u32, description = "House ID"),
        ("entity_name" = String, description = "Name of the solar entity"),
    )
)]
#[post("")]
async fn add_by_id(
    id: web::Path<(u32, String)>,
    _params: web::Json<SolarEntityParams>,
) -> impl Responder {
    let (house_id, entity_name) = id.into_inner();

    match demkit::env::add_solar(house_id).await {
        Ok(_) => HttpResponse::Ok().body(format!("{entity_name} added successfully")),
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    };

    HttpResponse::Ok().body(format!("{entity_name} added successfully"))
}

#[utoipa::path(
    delete,
    tag = "Solar",
    description = "Remove a solar entity.",
    responses(
        (status = 200, description = "Remove solar entity successfully"),
        (status = 500, description = "Failed to remove solar entity"),
    ),
    params(
        ("house_id" = u32, description = "House ID"),
        ("entity_name" = String, description = "Name of the solar entity"))
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
    tag = "Solar",
    description = "Get solar information.",
    responses(
        (status = 200, description = "Toggle solar state successfully"),
        (status = 500, description = "Failed to toggle solar state"),
    ),
    params(
        ("house_id" = u32, description = "House ID"),
        ("solar_id" = u32, description = "Solar ID"),
        ("state" = bool, description = "State to toggle to"),
    ),
)]
#[get("/toggle/{state}")]
async fn toggle(id: web::Path<(u32, u32, bool)>) -> impl Responder {
    let (house_id, _solar_id, state) = id.into_inner();
    demkit::solar::set_solar_state(house_id, state).await.unwrap();
    HttpResponse::Ok().body(format!("Toggled {state}"))
}