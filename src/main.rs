use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;

mod api;
mod resources;

use resources::house;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(api::health)
            .service(api::get_entity_consumption)
            .service(api::add_entity)
            .configure(house::configure)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
