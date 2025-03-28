use crate::api::demkit::ha_entity;
use crate::api::ha::entity;
use actix_web::{web, HttpResponse, Responder};

pub mod demkit;
pub mod ha;

#[actix_web::get("/healthz")]
pub async fn health() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().body("OK")
}


