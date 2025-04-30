use actix_web::get;

pub mod demkit;
pub mod ha;
pub mod docs;

#[utoipa::path(
    get,
    path = "/healthz",
    responses(
        (status = OK, description = "Health check OK"),
    )
)]
#[get("/healthz")]
pub async fn health() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().body("OK")
}