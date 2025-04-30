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

    let api_doc = api::docs::get_openapi();

    println!("{}", api_doc.to_pretty_json().unwrap());

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(api::health)
            .configure(house::configure)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
