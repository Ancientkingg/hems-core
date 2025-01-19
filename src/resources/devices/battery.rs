use actix_web::{get, web, HttpResponse, Responder};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/battery/{id}").service(get_by_id));
}

#[get("")]
async fn get_by_id(path: web::Path<(u32, u32)>) -> impl Responder {
    HttpResponse::Ok().body(format!("Battery id: {}", path.1))
}