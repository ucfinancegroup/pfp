use crate::models::user_model::User;
use crate::services::plans::PlansService::get_asset_classes_and_default_apys;
use actix_web::{get, HttpResponse};

#[get("/asset_classes")]
pub async fn get_asset_classes(_: User) -> HttpResponse {
    crate::common::into_response(get_asset_classes_and_default_apys())
}

use actix_web::web::ServiceConfig;
pub fn init_routes(config: &mut ServiceConfig) {
    config.service(get_asset_classes);
}
