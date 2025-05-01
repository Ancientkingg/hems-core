use utoipa::OpenApi;


#[derive(OpenApi)]
#[openapi(
    info(
        title = "HEMS-Core API",
        version = "1.0",
        description = "API documentation for the HEMS project as part of Convergence initiative.",
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