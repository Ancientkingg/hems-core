pub mod demkit;

#[actix_web::get("/")]
pub async fn health() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().body("Hello world!")
}