#[allow(non_snake_case)]
pub mod InsightsService {
  use crate::models::{insight_model::Insight, user_model::User};
  use crate::services::{db::DatabaseService, finchplaid};
  use chrono::{Duration, Utc};
  use wither::{
    mongodb::{
      bson::doc,
      options::{FindOneAndUpdateOptions, ReturnDocument},
    },
    Model,
  };

  pub async fn run_insights_service(
    db_service: DatabaseService,
    plaid_client: finchplaid::ApiClient,
  ) -> std::io::Result<()> {
    let one_day_ago = (Utc::now() - Duration::days(1)).timestamp();

    // get a user that needs new insight
    // based on: all insights over a day old, or none at all
    // TODO(c650) -- make this more complicated
    let user = User::find_one_and_update(
      &db_service.db,
      doc!{
        "$or": vec![
          doc!{"insights": doc!{"$size": 0}},
          doc!{"insights": doc!{"$not" : doc!{"$elemMatch" : doc!{"generation_time" : doc!{"$gte" : one_day_ago }}}}}
        ]
      },
      doc!{"$push": doc!{ "insights" : crate::common::into_bson_document(&Insight::default())}},
      FindOneAndUpdateOptions::builder()
        .return_document(ReturnDocument::After)
        .build(),
    )
    .await;

    println!("{:?}", user);

    if let Ok(Some(u)) = user {
      println!("{}", u.email.as_str());
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {

  #[test]
  fn test_reee() {}
}
