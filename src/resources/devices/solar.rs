use actix_web::{delete, get, post, web, HttpResponse, Responder};
use serde::Serialize;

use crate::api::demkit::{self, env::SolarEntityParams};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/solar/{id}")
            .service(get_by_id)
            .service(add_by_id)
            .service(remove_by_id)
            .service(toggle),
    );
}

#[derive(Serialize)]
struct SolarInfo {
    consumption: f64
}

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

#[delete("")]
async fn remove_by_id(id: web::Path<(u32, String)>) -> impl Responder {
    let (house_id, entity_name) = id.into_inner();

    match demkit::env::remove_entity(house_id, entity_name.as_str()).await {
        Ok(_) => HttpResponse::Ok().body(format!("{entity_name} removed successfully")),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    }
}

#[get("/toggle/{state}")]
async fn toggle(id: web::Path<(u32, u32, bool)>) -> impl Responder {
    let (house_id, _solar_id, state) = id.into_inner();
    demkit::solar::set_solar_state(house_id, state).await.unwrap();
    HttpResponse::Ok().body(format!("Toggled {state}"))
}