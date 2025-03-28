pub mod demkit;
pub mod ha;

#[actix_web::get("/healthz")]
pub async fn health() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().body("OK")
}