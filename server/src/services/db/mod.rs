use mongodb::bson::doc;
use mongodb::sync::{Client, Database};

pub fn connect_to_mongo(
  uri: String,
  db_user: String,
  db_pw: String,
  db_name: String,
) -> Result<Database, String> {
  let connection_str = format!(
    "mongodb+srv://{}:{}@{}/{}?w=majority",
    db_user, db_pw, uri, db_name
  );

  Client::with_uri_str(&connection_str).map_or_else(
    |_| Err("Failed to Parse Client".to_string()),
    |client| {
      let db = client.database(&db_name);

      db.run_command(doc! {"ping": 1}, None)
        .map_or_else(|_| Err("Fail".to_string()), |_| Ok(db))
    },
  )
}
