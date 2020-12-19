use mongodb::bson::doc;
use mongodb::sync::{Client, Collection, Database};

#[derive(Clone)]
pub struct DatabaseService {
  db: Database,
}

pub enum Collections {
  Users,
  Sessions,
  Snapshots,
}

impl DatabaseService {
  pub fn new(uri: String, db_user: String, db_pw: String, db_name: String) -> DatabaseService {
    let connection_str = format!(
      "mongodb+srv://{}:{}@{}/{}?w=majority",
      db_user, db_pw, uri, db_name
    );

    Client::with_uri_str(&connection_str)
      .map_or_else(
        |_| Err("Failed to resolve connection string to database".to_string()),
        |client| {
          let db = client.database(&db_name);

          db.run_command(doc! {"ping": 1}, None).map_or_else(
            |_| Err("Failed to run command ping".to_string()),
            |_| Ok(db),
          )
        },
      )
      .map_or_else(|error| panic!(error), |db: Database| DatabaseService { db })
  }

  // Lets you safely access a collection using an enum so that access collections
  // are checked at compile time.
  pub fn collection(&self, collection_name: Collections) -> Collection {
    use Collections::*;
    self.db.collection(match collection_name {
      Users => "Users",
      Sessions => "Sessions",
      Snapshots => "Snapshots",
    })
  }
}
