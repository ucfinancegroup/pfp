#[allow(non_snake_case)]
pub mod PlansService {
    use crate::common::errors::ApiError;
    use crate::controllers::plans_controller::{PlanNewPayload, PlanUpdatePayload};
    use crate::models::plan_model::*;
    use crate::models::user_model::User;
    use crate::services::users::UserService;
    use actix_web::web::Data;

    pub async fn new_plan(
        payload: PlanNewPayload,
        mut User: User,
        user_service: Data<UserService>,
    ) -> Result<Plan, ApiError> {
    }
}
