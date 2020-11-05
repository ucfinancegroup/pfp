use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {

}

impl User {
  #[allow(dead_code)]
  pub fn new() -> User { User {} }


}
