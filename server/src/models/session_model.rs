use mongodb::{
  bson::{bson, doc, oid::ObjectId},
  sync::Collection,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
  _id: ObjectId,
  user_id: ObjectId,
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
            _id: sid_obj,
            user_id: user_id_obj,
          })
        } else {
          None
        }
      }
      _ => None,
    }
  }

  pub fn new_user_session(
    col: Collection,
    user_id: ObjectId,
    session: &actix_session::Session,
  ) -> Session {
    let ret = Session {
      _id: ObjectId::new(),
      user_id: user_id,
    };
    let _ = session.set("sid", ret._id.to_hex());
    let _ = session.set("user_id", ret.user_id.to_hex());
    let _ = col.insert_one(bson!(ret.clone()).as_document().unwrap().clone(), None);

    ret
  }

  pub fn is_valid(&self, col: Collection) -> bool {
    col
      .find_one(
        Some(bson!(self.clone()).as_document().unwrap().clone()),
        None,
      )
      .map_or_else(|_| false, |got| got.is_some())
  }

  pub fn invalidate(&self, col: Collection) -> () {
    let _ = col.delete_one(doc! {"_id" : self._id.clone()}, None);
  }
}

impl std::convert::From<Session> for mongodb::bson::Bson {
  fn from(s: Session) -> mongodb::bson::Bson {
    mongodb::bson::to_bson(&s).unwrap()
  }
}
