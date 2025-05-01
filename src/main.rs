use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;

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
            .openapi(api::docs::get_openapi())
            .map(|app| app.wrap(Logger::default()))
            .service(api::health)
            .configure(house::configure)
            .openapi_service(|api| {
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", api)
            })
            .split_for_parts();

        let api_doc = api.to_pretty_json().unwrap();
        std::fs::create_dir_all("./api-docs").unwrap();
        std::fs::write("./api-docs/api-doc.json", api_doc).unwrap();

        app
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
