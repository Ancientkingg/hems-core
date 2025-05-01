use utoipa::OpenApi;


#[derive(OpenApi)]
#[openapi(
    info(
        title = "Demkit API",
        version = "1.0",
        description = "API documentation for the Demkit project",
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        ),
    ),
)]
pub struct ApiDoc;

pub fn get_openapi() -> utoipa::openapi::OpenApi {
    ApiDoc::openapi()
}