#[allow(non_snake_case)]
pub mod PlansService {
    use crate::common::errors::ApiError;
    use crate::controllers::plans_controller::PlanNewPayload;
    use crate::models::plan_model::*;
    use crate::models::user_model::User;
    use crate::services::users::UserService;
    use actix_web::web::Data;
    use wither::{mongodb::bson::oid::ObjectId, Model};

    pub async fn new_plan(
        payload: PlanNewPayload,
        mut user: User,
        user_service: Data<UserService>,
    ) -> Result<Plan, ApiError> {
        let mut plan: Plan = payload.into();
        plan.set_id(ObjectId::new());

        user.plans.push(plan.clone());

        user_service.save(&mut user).await?;

        Ok(plan)
    }
}
