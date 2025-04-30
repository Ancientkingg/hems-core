use crate::api::demkit::ha_entity::{self, EntityRequest};
use crate::api::ha::entity;
use actix_web::{get, post, web, HttpResponse, Responder};
use utoipa_actix_web::scope;

pub fn configure(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(
        scope::scope("/entity")
            .service(get_entity_consumption)
            .service(add_entity),
    );
}

#[utoipa::path(
    get,
    path = "/entity/{entity_name}/consumption",
    responses(
        (status = 200, description = "Get entity consumption", body = EntityRequest),
        (status = 500, description = "Failed to get entity consumption"),
    ),
)]
#[get("/{entity_name}/consumption")]
async fn get_entity_consumption(path: web::Path<(String, String)>) -> impl Responder {
    let entity_name = path.into_inner().1;
    match entity::get_entity_consumption(&entity_name).await {
        Ok(entity_state) => HttpResponse::Ok().json(entity_state),
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("Failed to get device consumption: {}", e)),
    }
}

#[utoipa::path(
    post,
    path = "/entity",
    request_body = EntityRequest,
    responses(
        (status = 200, description = "Entity added successfully"),
        (status = 500, description = "Failed to add entity"),
    ),
)]
#[post("")]
async fn add_entity(request: web::Json<EntityRequest>) -> impl Responder {
    let entity = request.into_inner();
    match ha_entity::add_entity(entity).await {
        Ok(_) => HttpResponse::Ok().body("OK"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to add entity, :{}", e)),
    }
}
