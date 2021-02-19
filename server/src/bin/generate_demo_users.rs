use chrono::Utc;
use pfp_server::common::Money;
use pfp_server::models::user_model::{Location, Snapshot, User};
use rust_decimal_macros::dec;
use serde_json::to_string_pretty;
use std::fs;

fn main() {
  let default_password = User::hash_password("password".to_string()).unwrap();

  let first_snapshot_time = (Utc::now() - chrono::Duration::days(3)).timestamp();
  let second_snapshot_time = Utc::now().timestamp();

  let demo_users: Vec<User> = vec![
    User {
      id: None,
      email: "hamilton@us.gov".to_string(),
      password: default_password.clone(),
      first_name: "Alexander".to_string(),
      last_name: "Hamilton".to_string(),
      income: dec!(30_000.0),
      net_worth: dec!(10_000_000.00),
      location: Location {
        // new york city
        has_location: true,
        lat: 40.7128,
        lon: 74.0060,
      },
      birthday: "1755-01-11".to_string(),
      accounts: vec![],
      account_records: vec![],
      snapshots: vec![
        Snapshot {
          net_worth: Money::new(dec!(1_000_000)),
          running_income: Money::new(dec!(0)),
          running_savings: Money::new(dec!(0)),
          running_spending: Money::new(dec!(0)),
          snapshot_time: first_snapshot_time,
        },
        Snapshot {
          net_worth: Money::new(dec!(1_000_000)),
          running_income: Money::new(dec!(1_000)),
          running_savings: Money::new(dec!(0)),
          running_spending: Money::new(dec!(1_000)),
          snapshot_time: second_snapshot_time,
        },
      ],
      recurrings: vec![],
      goals: vec![],
      insights: vec![],
      plans: vec![],
    },
    User {
      id: None,
      email: "jefferson@us.gov".to_string(),
      password: default_password.clone(),
      first_name: "Thomas".to_string(),
      last_name: "Jefferson".to_string(),
      income: dec!(30_000.0),
      net_worth: dec!(12_000_000.00),
      location: Location {
        // monticello
        has_location: true,
        lat: 38.0086,
        lon: 78.4532,
      },
      birthday: "1743-04-13".to_string(),
      accounts: vec![],
      account_records: vec![],
      snapshots: vec![
        Snapshot {
          net_worth: Money::new(dec!(1_000_000)),
          running_income: Money::new(dec!(0)),
          running_savings: Money::new(dec!(0)),
          running_spending: Money::new(dec!(0)),
          snapshot_time: first_snapshot_time,
        },
        Snapshot {
          net_worth: Money::new(dec!(1_001_000)),
          running_income: Money::new(dec!(2_000)),
          running_savings: Money::new(dec!(1_000)),
          running_spending: Money::new(dec!(1_000)),
          snapshot_time: second_snapshot_time,
        },
      ],
      recurrings: vec![],
      goals: vec![],
      insights: vec![],
      plans: vec![],
    },
    User {
      id: None,
      email: "washington@us.gov".to_string(),
      password: default_password.clone(),
      first_name: "George".to_string(),
      last_name: "Washington".to_string(),
      income: dec!(30_000.0),
      net_worth: dec!(22_000_000.00),
      location: Location {
        // mt vernon
        has_location: true,
        lat: 48.4201,
        lon: 122.3375,
      },
      birthday: "1732-02-22".to_string(),
      accounts: vec![],
      account_records: vec![],
      snapshots: vec![
        Snapshot {
          net_worth: Money::new(dec!(1_000_000)),
          running_income: Money::new(dec!(0)),
          running_savings: Money::new(dec!(0)),
          running_spending: Money::new(dec!(0)),
          snapshot_time: first_snapshot_time,
        },
        Snapshot {
          net_worth: Money::new(dec!(1_040_000)),
          running_income: Money::new(dec!(40_000)),
          running_savings: Money::new(dec!(1_000)),
          running_spending: Money::new(dec!(1_000)),
          snapshot_time: second_snapshot_time,
        },
      ],
      recurrings: vec![],
      goals: vec![],
      insights: vec![],
      plans: vec![],
    },
  ];

  let s = to_string_pretty(&demo_users).unwrap();
  let _ = fs::write("file", s).unwrap();
}
