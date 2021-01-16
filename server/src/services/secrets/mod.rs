use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize)]
pub struct Environment {
  pub database_url: String,
  pub database_user: String,
  pub database_pw: String,
  pub database_name: String,
  pub plaid_client_id: String,
  pub plaid_sandbox_secret: String,
}

impl Environment {
  pub fn new() -> Result<Environment, Box<dyn Error>> {
    let file =
      File::open("/run/secrets/config.json").or(File::open("/etc/k8s_secrets/config.json")).or(File::open("./config.json"))?;
    let reader = BufReader::new(file);
    let env = serde_json::from_reader(reader)?;
    Ok(env)
  }
}
