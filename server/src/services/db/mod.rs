use futures::TryFutureExt;
use wither::mongodb::bson::doc;
use wither::mongodb::{Client, Database};

#[derive(Clone)]
pub struct DatabaseService {
  pub db: Database,
}

impl DatabaseService {
  pub async fn new(
    uri: String,
    db_user: String,
    db_pw: String,
    db_name: String,
  ) -> DatabaseService {
    let connection_str = format!(
      "mongodb+srv://{}:{}@{}/{}?w=majority",
      db_user, db_pw, uri, db_name
    );

    let db = Client::with_uri_str(&connection_str)
      .map_ok_or_else(
        |_| Err("Failed to resolve connection string to database".to_string()),
        |client| Ok(client.database(&db_name)),
      )
      .await
      .unwrap();

    db.run_command(doc! {"ping": 1}, None)
      .map_ok_or_else(
        |_| Err("Failed to run command ping".to_string()),
        |_| Ok(DatabaseService { db: db.clone() }),
      )
      .await
      .unwrap()
  }
}
