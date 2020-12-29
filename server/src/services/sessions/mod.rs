use crate::common::errors::ApiError;
use crate::models::{session_model::*, user_model::User};
use crate::services::db;
use wither::{mongodb::Database, Model};

#[derive(Clone)]
pub struct SessionService {
  db: Database,
}

impl SessionService {
  pub fn new(db: &db::DatabaseService) -> SessionService {
    SessionService { db: db.db.clone() }
  }

  pub async fn new_user_session(
    &self,
    user: &User,
    session: &actix_session::Session,
  ) -> Result<(), ApiError> {
    let user_id = user.id.clone().unwrap();

    let mut new_session = Session {
      id: None,
      user_id: user_id,
    };

    new_session
      .save(&self.db, None)
      .await
      .map_err(|_| ApiError::new(500, "Database Error".to_string()))?;

    let _ = session.set("sid", new_session.id.unwrap().to_hex());
    let _ = session.set("user_id", new_session.user_id.to_hex());

    Ok(())
  }

  pub async fn get_valid_session(
    &self,
    session: &actix_session::Session,
  ) -> Result<Session, ApiError> {
    let finch_session =
      Session::new_from_store(&session).ok_or(ApiError::new(401, "No session".to_string()))?;
    match self.is_valid(&finch_session).await {
      true => Ok(finch_session),
      false => Err(ApiError::new(401, "Invalid session".to_string()).into()),
    }
  }

  pub async fn invalidate(&self, session: &Session) -> () {
    let _ = session.delete(&self.db).await;
  }

  async fn is_valid(&self, session: &Session) -> bool {
    Session::find_one(
      &self.db,
      Some(session.document_from_instance().unwrap()),
      None,
    )
    .await
    .map_or_else(|_| false, |got| got.is_some())
  }
}
