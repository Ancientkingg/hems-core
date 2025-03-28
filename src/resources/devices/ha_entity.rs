use crate::api::demkit::ha_entity;
use crate::api::ha::entity;
use actix_web::{web, HttpResponse, Responder};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/entity")
            .service(get_entity_consumption)
            .service(add_entity),
    );
}

#[derive(serde::Deserialize)]
struct EntityRequest {
    entity_id: String,
}

#[actix_web::get("/{entity_name}/consumption")]
async fn get_entity_consumption(entity_name: web::Path<String>) -> impl Responder {
    let entity_state = entity::get_entity_consumption(&entity_name).await;

    if entity_state.is_err() {
        return HttpResponse::InternalServerError().body("Failed to get device consumption");
    }

    return HttpResponse::Ok().json(entity_state.unwrap());
}

#[actix_web::post("")]
async fn add_entity(request: web::Json<EntityRequest>) -> impl Responder {
    let entity_id = &request.entity_id;
    if ha_entity::add_entity(entity_id).await.is_err() {
        return HttpResponse::InternalServerError().body("Failed to add entity");
    }
    return HttpResponse::Ok().body("OK");
}
