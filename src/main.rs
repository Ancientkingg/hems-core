use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;
use utoipa_actix_web::AppExt;

mod api;
mod resources;

use resources::house;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        let (app, api) = App::new()
        .into_utoipa_app()
        .service(api::health)
        .configure(house::configure)
        .split_for_parts();

        let api_doc = api.to_pretty_json().unwrap();
        std::fs::write("api-doc.json", api_doc).unwrap();

        app.wrap(Logger::default())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
