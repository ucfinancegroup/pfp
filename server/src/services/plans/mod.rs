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

    pub async fn get_plan(plan_id: String, user: User) -> Result<Plan, ApiError> {
        let plan_id_opt = Some(
            ObjectId::with_string(plan_id.as_str())
                .or(Err(ApiError::new(400, "Malformed Object Id".to_string())))?,
        );

        let found = user
            .plans
            .into_iter()
            .find(|rec| rec.id == plan_id_opt)
            .clone();

        found.ok_or(ApiError::new(
            400,
            format!("No plan with id {} found in current user", plan_id),
        ))
    }

    pub async fn update_plan(
        plan_id: String,
        payload: PlanNewPayload,
        mut user: User,
        user_service: Data<UserService>,
    ) -> Result<Plan, ApiError> {
        let plan_id_opt = Some(
            ObjectId::with_string(plan_id.as_str())
                .or(Err(ApiError::new(400, "Malformed Object Id".to_string())))?,
        );
        let mut plan: Plan = payload.into();
        plan.set_id(plan_id_opt.unwrap().clone());
        let updated = user
            .plans
            .iter_mut()
            .find(|rec| rec.id == plan.id)
            .ok_or(ApiError::new(
                400,
                format!("No plan with id {} found in current user", plan_id),
            ))
            .and_then(|rec| {
                *rec = plan.clone();
                Ok(plan)
            })?;
        user_service.save(&mut user).await?;
        Ok(updated)
    }

    pub async fn delete_plan(
        plan_id: String,
        mut user: User,
        user_service: Data<UserService>,
    ) -> Result<Plan, ApiError> {
        let plan_id_opt = Some(
            ObjectId::with_string(plan_id.as_str())
                .or(Err(ApiError::new(400, "Malformed Object Id".to_string())))?,
        );
        let removed = user
            .plans
            .iter()
            .position(|rec| rec.id == plan_id_opt)
            .ok_or(ApiError::new(
                400,
                format!("No plan with id {} found in current user", plan_id),
            ))
            .and_then(|pos| Ok(user.plans.swap_remove(pos)))?;
        user_service.save(&mut user).await?;

        Ok(removed)
    }
}
