use wither::{mongodb::bson::oid::ObjectId, Model};

use serde::{Deserialize, Serialize};

#[derive(Model, Serialize, Deserialize, Debug, Clone)]
pub struct Session {
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<ObjectId>,
  pub user_id: ObjectId,
}

impl Session {
  pub fn new_from_store(session: &actix_session::Session) -> Option<Session> {
    let sid: Result<Option<String>, actix_web::Error> = session.get("sid");
    let user_id: Result<Option<String>, actix_web::Error> = session.get("user_id");

    match (sid, user_id) {
      (Ok(Some(sid_str)), Ok(Some(user_id_str))) => {
        if let (Ok(sid_obj), Ok(user_id_obj)) = (
          ObjectId::with_string(sid_str.as_str()),
          ObjectId::with_string(user_id_str.as_str()),
        ) {
          Some(Session {
            id: Some(sid_obj),
            user_id: user_id_obj,
          })
        } else {
          None
        }
      }
      _ => None,
    }
  }
}

#[allow(unused_imports)]
use chrono::TimeZone;
use wither::mongodb::bson::doc;
use wither::prelude::Migrating;

impl Migrating for Session {
  // Define any migrations which your model needs in this method.
  // As this is an interval migration, it will deactivate itself after the given threshold
  // date, so you could leave it in your code for as long as you would like.
  fn migrations() -> Vec<Box<dyn wither::Migration>> {
    // -- EXAMPLE --
    // vec![Box::new(wither::IntervalMigration {
    //   name: "remove-oldfield".to_string(),
    //   // NOTE: use a logical time here. A day after your deployment date, or the like.
    //   threshold: chrono::Utc.ymd(2100, 1, 1).and_hms(1, 0, 0),
    //   filter: doc! {"oldfield": doc!{"$exists": true}},
    //   set: None,
    //   unset: Some(doc! {"oldfield": ""}),
    // })]
    vec![]
  }
}
