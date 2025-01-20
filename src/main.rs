use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use api::demkit::get_energy_import;
use env_logger::Env;

mod api;
mod resources;

use resources::house;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/test")]
async fn test() -> impl Responder {

    let import = get_energy_import(0).await.unwrap();

    let (energy, unit) = (import.value, import.unit);

    HttpResponse::Ok().body(format!("Energy: {energy} {unit}"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(hello)
            .service(test)
            .configure(house::configure)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}