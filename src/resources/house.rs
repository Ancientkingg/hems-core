use actix_web::{get, post, web, HttpResponse, Responder};

#[path = "devices/devices.rs"]
mod devices;
use devices::{battery, meter, solar, thermal};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/houses/{id}")
            .service(get_by_id)
            .configure(battery::configure)
            .configure(meter::configure)
            .configure(solar::configure)
            .configure(thermal::configure),
    );
}

#[get("")]
async fn get_by_id(path: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("House id: {}", path))
}
