#[allow(non_snake_case)]
pub mod GoalService {
  use crate::common::errors::ApiError;
  use crate::controllers::goal_controller::GoalNewPayload;
  use crate::models::{
    goal_model::*,
    user_model::{Snapshot, User},
  };
  use crate::services::users::UserService;
  use actix_web::web::Data;
  use wither::{mongodb::bson::oid::ObjectId, Model};

  pub fn calculate_goal_progress(
    goal: Goal,
    snapshots: &Vec<Snapshot>,
  ) -> Result<GoalAndStatus, ApiError> {
    let last_before_start = snapshots.iter().fold(Snapshot::default(), |acc, cur| {
      if cur.snapshot_time < goal.start {
        cur.clone()
      } else {
        acc
      }
    });

    let last_in_range = snapshots.iter().fold(Snapshot::default(), |acc, cur| {
      if cur.snapshot_time > goal.end {
        acc
      } else {
        cur.clone()
      }
    });

    let change = match goal.metric {
      GoalMetrics::Income => {
        last_in_range.running_income.amount - last_before_start.running_income.amount
      }
      GoalMetrics::Savings => {
        last_in_range.running_savings.amount - last_before_start.running_savings.amount
      }
      GoalMetrics::Spending => {
        last_in_range.running_spending.amount - last_before_start.running_spending.amount
      }
    };

    if change == goal.threshold {
      return Ok(GoalAndStatus {
        progress: 100.0,
        goal,
      });
    }

    Ok(GoalAndStatus {
      progress: (change as f64) / (goal.threshold.clone() as f64),
      goal,
    })
  }

  pub async fn get_goal(goal_id: String, user: User) -> Result<GoalAndStatus, ApiError> {
    let goal = retrieve_goal(goal_id, user.goals)?;
    calculate_goal_progress(goal, &user.snapshots)
  }

  pub async fn get_all_goals(user: User) -> Result<Vec<GoalAndStatus>, ApiError> {
    user
      .goals
      .iter()
      .map(|goal| calculate_goal_progress(goal.clone(), &user.snapshots))
      .collect::<Result<Vec<GoalAndStatus>, ApiError>>()
  }

  pub fn retrieve_goal(goal_id: String, goals: Vec<Goal>) -> Result<Goal, ApiError> {
    let goal_id_opt = Some(
      ObjectId::with_string(goal_id.as_str())
        .or(Err(ApiError::new(400, "Malformed Object Id".to_string())))?,
    );

    let found = goals.into_iter().find(|rec| rec.id == goal_id_opt).clone();

    found.ok_or(ApiError::new(
      400,
      format!("No goal with id {} found in current user", goal_id),
    ))
  }

  pub async fn new_goal(
    payload: GoalNewPayload,
    mut user: User,
    user_service: Data<UserService>,
  ) -> Result<GoalAndStatus, ApiError> {
    let mut goal: Goal = payload.into();
    goal.set_id(ObjectId::new());

    user.goals.push(goal.clone());

    user_service.save(&mut user).await?;

    calculate_goal_progress(goal, &user.snapshots)
  }

  pub async fn update_goal(
    goal_id: String,
    payload: GoalNewPayload,
    mut user: User,
    user_service: Data<UserService>,
  ) -> Result<GoalAndStatus, ApiError> {
    let goal_id_opt = Some(
      ObjectId::with_string(goal_id.as_str())
        .or(Err(ApiError::new(400, "Malformed Object Id".to_string())))?,
    );

    let mut goal: Goal = payload.into();
    goal.set_id(goal_id_opt.unwrap().clone());

    let updated = user
      .goals
      .iter_mut()
      .find(|rec| rec.id == goal.id)
      .ok_or(ApiError::new(
        400,
        format!("No goal with id {} found in current user", goal_id),
      ))
      .and_then(|rec| {
        *rec = goal.clone();
        Ok(goal)
      })?;

    user_service.save(&mut user).await?;

    calculate_goal_progress(updated, &user.snapshots)
  }

  pub async fn delete_goal(
    goal_id: String,
    mut user: User,
    user_service: Data<UserService>,
  ) -> Result<GoalAndStatus, ApiError> {
    let goal_id_opt = Some(
      ObjectId::with_string(goal_id.as_str())
        .or(Err(ApiError::new(400, "Malformed Object Id".to_string())))?,
    );

    let removed = user
      .goals
      .iter()
      .position(|rec| rec.id == goal_id_opt)
      .ok_or(ApiError::new(
        400,
        format!("No goal with id {} found in current user", goal_id),
      ))
      .and_then(|pos| Ok(user.goals.swap_remove(pos)))?;

    user_service.save(&mut user).await?;

    calculate_goal_progress(removed, &user.snapshots)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use crate::common::Money;
  use crate::models::{goal_model::*, user_model::Snapshot};

  #[test]
  fn test_calculate_progress() {
    let saving_goal = Goal {
      id: None,
      name: "Save 100 Dollars".to_string(),
      start: 3,
      end: 10,
      threshold: 10000, // dollars times 100
      metric: GoalMetrics::Savings,
    };

    let snapshots = vec![
      Snapshot {
        net_worth: Money { amount: 0 },
        running_spending: Money { amount: 0 },
        running_savings: Money { amount: 0 },
        running_income: Money { amount: 0 },
        snapshot_time: 1,
      },
      Snapshot {
        net_worth: Money { amount: 0 },
        running_spending: Money { amount: 0 },
        running_savings: Money { amount: 0 },
        running_income: Money { amount: 0 },
        snapshot_time: 2,
      },
      Snapshot {
        net_worth: Money { amount: 0 },
        running_spending: Money { amount: 0 },
        running_savings: Money { amount: 0 },
        running_income: Money { amount: 0 },
        snapshot_time: 4,
      },
      Snapshot {
        net_worth: Money { amount: 0 },
        running_spending: Money { amount: -1000 },
        running_savings: Money { amount: 5000 },
        running_income: Money { amount: 0 },
        snapshot_time: 6,
      },
      Snapshot {
        net_worth: Money { amount: 0 },
        running_spending: Money { amount: -1000 },
        running_savings: Money { amount: 5000 },
        running_income: Money { amount: 0 },
        snapshot_time: 7,
      },
      Snapshot {
        net_worth: Money { amount: 0 },
        running_spending: Money { amount: -1000 },
        running_savings: Money { amount: 5000 },
        running_income: Money { amount: 0 },
        snapshot_time: 10,
      },
      Snapshot {
        net_worth: Money { amount: 0 },
        running_spending: Money { amount: -21000 },
        running_savings: Money { amount: 5000 },
        running_income: Money { amount: 0 },
        snapshot_time: 11,
      },
    ];

    let progress = GoalService::calculate_goal_progress(saving_goal, &snapshots).unwrap();

    assert_eq!(0.5 as f64, progress.progress);

    let spending_goal = Goal {
      id: None,
      name: "Spend under 100 Dollars".to_string(),
      start: 5,
      end: 9,
      threshold: -10000, // dollars times 100
      metric: GoalMetrics::Spending,
    };

    let progress2 = GoalService::calculate_goal_progress(spending_goal, &snapshots).unwrap();

    assert_eq!(0.1 as f64, progress2.progress);

    let spending_goal2 = Goal {
      id: None,
      name: "Spend under 100 Dollars".to_string(),
      start: 6,
      end: 12,
      threshold: -10000, // dollars times 100
      metric: GoalMetrics::Spending,
    };

    let progress3 = GoalService::calculate_goal_progress(spending_goal2, &snapshots).unwrap();

    assert_eq!(2.1 as f64, progress3.progress);
  }

  #[test]
  fn test_calculate_progress_no_snapshots() {
    let goal = Goal {
      id: None,
      name: "Spend under 100 Dollars".to_string(),
      start: 6,
      end: 12,
      threshold: -10000, // dollars times 100
      metric: GoalMetrics::Spending,
    };

    let progress = GoalService::calculate_goal_progress(goal, &vec![]).unwrap();

    // no snapshots should complete successfully and return _no_ progress
    assert_eq!(0.0 as f64, progress.progress);
  }
}
