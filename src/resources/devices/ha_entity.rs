use std::fmt::format;

use crate::api::demkit::ha_entity::{self, EntityRequest};
use crate::api::ha::entity;
use actix_web::{web, HttpResponse, Responder};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/entity")
            .service(get_entity_consumption)
            .service(add_entity),
    );
}

#[actix_web::get("/{entity_name}/consumption")]
async fn get_entity_consumption(path: web::Path<(String, String)>) -> impl Responder {
    let entity_name = path.into_inner().1;
    match entity::get_entity_consumption(&entity_name).await {
        Ok(entity_state) => HttpResponse::Ok().json(entity_state),
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("Failed to get device consumption: {}", e)),
    }
}

#[actix_web::post("")]
async fn add_entity(request: web::Json<EntityRequest>) -> impl Responder {
    let entity = request.into_inner();
    match ha_entity::add_entity(entity).await {
        Ok(_) => HttpResponse::Ok().body("OK"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to add entity, :{}", e)),
    }
}
