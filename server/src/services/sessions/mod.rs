use crate::common;
use crate::common::errors::ApiError;
use crate::models::session_model::*;
use crate::services::db;

use mongodb::{
  bson::{doc, oid::ObjectId},
  sync::Collection,
};

#[derive(Clone)]
pub struct SessionService {
  col: Collection,
}

impl SessionService {
  pub fn new(db: &db::DatabaseService) -> SessionService {
    SessionService {
      col: db.collection(db::Collections::Sessions),
    }
  }

  pub fn new_user_session(&self, user_id: ObjectId, session: &actix_session::Session) -> Session {
    let new_session = Session {
      _id: ObjectId::new(),
      user_id: user_id,
    };
    let _ = session.set("sid", new_session._id.to_hex());
    let _ = session.set("user_id", new_session.user_id.to_hex());
    let _ = self
      .col
      .insert_one(common::into_bson_document(&new_session), None);

    new_session
  }

  pub fn get_valid_session(&self, session: &actix_session::Session) -> Result<Session, ApiError> {
    Session::new_from_store(&session).map_or_else(
      || Err(ApiError::new(401, "No session".to_string())),
      |finch_session| match self.is_valid(&finch_session) {
        true => Ok(finch_session),
        false => Err(ApiError::new(401, "Invalid session".to_string()).into()),
      },
    )
  }

  pub fn invalidate(&self, session: &Session) -> () {
    let _ = self
      .col
      .delete_one(doc! {"_id" : session._id.clone()}, None);
  }

  fn is_valid(&self, session: &Session) -> bool {
    self
      .col
      .find_one(Some(common::into_bson_document(session)), None)
      .map_or_else(|_| false, |got| got.is_some())
  }
}
