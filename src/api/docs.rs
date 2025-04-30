use utoipa::OpenApi;

use crate::resources::house::devices;


#[derive(OpenApi)]
#[openapi(paths(devices::timeshifters::get_by_id))]
pub struct ApiDoc;

pub fn get_openapi() -> utoipa::openapi::OpenApi {
    ApiDoc::openapi()
}