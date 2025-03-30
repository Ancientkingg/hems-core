use actix_web::{get, post, web, HttpResponse, Responder};

#[path = "devices/devices.rs"]
mod devices;
use devices::{battery, ha_entity, meter, solar, thermal, timeshifters};

use crate::api::demkit;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/houses/{id}")
            .service(get_by_id)
            .service(get_time)
            .configure(battery::configure)
            .configure(meter::configure)
            .configure(solar::configure)
            .configure(thermal::configure)
            .configure(timeshifters::configure)
            .configure(ha_entity::configure),
    );
}

#[get("")]
async fn get_by_id(path: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("House id: {}", path))
    // list entities and show /composer
}

#[post("")]
async fn compose(path: web::Path<u32>) -> impl Responder {
    let _house_id = path.into_inner();
    match demkit::env::add_host().await {
        Ok(_) => println!("Host added successfully"),
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {}", e))
    }

    match demkit::env::add_weather().await {
        Ok(_) => println!("Weather added successfully"),
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {}", e))
    }

    match demkit::env::add_sun().await {
        Ok(_) => println!("Sun added successfully"),
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {}", e))
    }


    HttpResponse::Ok().body("House composed successfully")
}

#[post("/load")]
async fn load(path: web::Path<u32>) -> impl Responder {
    let _house_id = path.into_inner();
    match demkit::env::load().await {
        Ok(_) => println!("House loaded successfully"),
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {}", e))
    }

    match demkit::env::start().await {
        Ok(_) => println!("House started successfully"),
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {}", e))
    }

    HttpResponse::Ok().body("House simulation loaded successfully and currently running")
}

#[post("/config")]
async fn set_config(path: web::Path<u32>, config: web::Json<demkit::env::SimConfig>) -> impl Responder {
    todo!("sadsa");
}

#[get("/time")]
async fn get_time() -> impl Responder {
    let current_time = demkit::get_time().await;
    HttpResponse::Ok().body(current_time.to_string())
}

#[get("/entities")]
async fn list_entities() -> impl Responder {
    let entities = demkit::list_entities().await;
    HttpResponse::Ok().json(entities)
}