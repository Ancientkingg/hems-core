use actix_web::{get, web, HttpResponse, Responder};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/battery/{id}").service(get_by_id));
}

#[get("")]
async fn get_by_id(house_id: web::Path<u32>, battery_id: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("Battery {battery_id} in house {house_id}"))
}