#[allow(non_snake_case)]
pub mod SnapshotService {
  use crate::common::errors::ApiError;
  use crate::models::user_model::{PlaidItem, Snapshot, User};
  use crate::services::finchplaid::ApiClient;
  use actix_web::web::Data;
  use chrono::{Duration, Utc};
  use plaid::models::{RetrieveTransactionsRequest, RetrieveTransactionsResponse, Transaction};
  use std::sync::{Arc, Mutex};

  pub async fn add_new_snapshot(
    user: &mut User,
    plaid_client: Data<Arc<Mutex<ApiClient>>>,
  ) -> Result<(), ApiError> {
    // handle each item connected to user
    let mut per_item_stats = Vec::new();
    for item in user.accounts.iter() {
      per_item_stats.push(handle_item(item, plaid_client.clone()).await?);
    }

    // accumulate each item to a total
    let (total_money_in, total_money_out, total_net) = per_item_stats
      .iter()
      .fold((0.0, 0.0, 0.0), |(a, b, c), (e, f, g)| {
        (a + e, b + f, c + g)
      });

    // for rolling sums
    let prev = get_last_snapshot(&user.snapshots);

    // create the new snapshot
    let mut curr = Snapshot {
      net_worth: total_net.into(),
      running_savings: (total_money_in - total_money_out).into(),
      running_spending: total_money_out.into(),
      running_income: total_money_in.into(),
      snapshot_time: Utc::now().timestamp(),
    };

    // make it a cumulative sum
    curr.running_savings.amount += prev.running_savings.amount;
    curr.running_spending.amount += prev.running_spending.amount;
    curr.running_income.amount += prev.running_income.amount;

    user.snapshots.push(curr);

    Ok(())
  }

  pub async fn handle_item(
    item: &PlaidItem,
    plaid_client: Data<Arc<Mutex<ApiClient>>>,
  ) -> Result<(f64, f64, f64), ApiError> {
    // accumulate money_in and money_out for items' transactions
    let (money_in, money_out) = get_money_in_out(item, plaid_client.clone()).await?;

    // get net worth of items accounts
    match crate::services::finchplaid::get_net_worth(item, plaid_client).await {
      Ok(num) => return Ok((money_in, money_out, num)),
      Err(e) => return Err(e),
    };
  }

  async fn get_money_in_out(
    item: &PlaidItem,
    plaid_client: Data<Arc<Mutex<ApiClient>>>,
  ) -> Result<(f64, f64), ApiError> {
    let transactions = get_item_transactions_for_new_snapshot(item, plaid_client).await?;

    Ok(calculate_money_in_out(&transactions))
  }

  pub fn calculate_money_in_out(
    transactions_response: &RetrieveTransactionsResponse,
  ) -> (f64, f64) {
    // map each account to a coefficient for each transaction.
    let account_id_to_coeff = crate::services::finchplaid::get_account_transaction_coefficients(
      &transactions_response.accounts,
    );

    // accumulate money_in and money_out for transactions
    transactions_response.transactions.iter().fold(
      (0.0, 0.0),
      |(money_in, money_out), transaction: &Transaction| {
        let s: f64 = (transaction.amount as f64)
          * *account_id_to_coeff
            .get(&transaction.account_id)
            .or(Some(&0.0))
            .unwrap();
        (money_in + s.max(0.0), money_out + s.min(0.0))
      },
    )
  }

  async fn get_item_transactions_for_new_snapshot(
    item: &PlaidItem,
    plaid_client: Data<Arc<Mutex<ApiClient>>>,
  ) -> Result<RetrieveTransactionsResponse, ApiError> {
    // offset by 2 days to ensure we get a full day and avoid any timezone problems
    let date = (Utc::now() - chrono::Duration::days(2))
      .format("%Y-%m-%d")
      .to_string();

    // TODO: ensure we actually get all the transactions in a day. As of 2021 Jan 03,
    // the Plaid API returns up to 100 transactions by default. This should be enough
    // to cover one day of transactions for a normal person on one day.
    // ... but uhh maybe not thanks Robinhood.

    let pc = plaid_client.lock().unwrap();
    let config = &(pc.configuration);

    plaid::apis::transactions_api::retrieve_transactions(
      &config,
      RetrieveTransactionsRequest::new(
        pc.client_id.clone(),
        pc.secret.clone(),
        item.access_token.clone(),
        date.clone(),
        date.clone(),
      ),
    )
    .await
    .map_err(|_| ApiError::new(500, "Error while getting transactions".to_string()))
  }

  pub fn need_new_snapshot(snapshots: &Vec<Snapshot>) -> bool {
    let now = Utc::now().timestamp();
    let last_time = get_last_snapshot(snapshots).snapshot_time;

    println!("Last snapshot at {}. Currently it is {}", last_time, now);

    // need a new snapshot if the last one was more than a day ago
    Duration::seconds(now - last_time) > Duration::days(1)
  }

  fn get_last_snapshot(snapshots: &Vec<Snapshot>) -> Snapshot {
    if snapshots.len() <= 0 {
      return Snapshot::default();
    }
    snapshots[snapshots.len() - 1].clone()
  }
}

#[cfg(test)]
mod test {
  use super::*;

  use std::error::Error;
  use std::fs::File;
  use std::io::BufReader;

  use plaid::models::RetrieveTransactionsResponse;

  fn load_test_data() -> Result<RetrieveTransactionsResponse, Box<dyn Error>> {
    let file = File::open("./tests/test_snapshots.json")?;
    let reader = BufReader::new(file);
    let transactions = serde_json::from_reader(reader)?;
    Ok(transactions)
  }

  #[test]
  fn test_calculate_money_in_money_out() {
    let transactions = load_test_data().unwrap();
    assert_eq!(
      (0.0, -10965.230000019073) as (f64, f64),
      SnapshotService::calculate_money_in_out(&transactions)
    );
  }

  #[test]
  fn test_calculate_net_worth() {
    let transactions = load_test_data().unwrap();
    assert_eq!(
      -53501.318115234375 as f64,
      crate::services::finchplaid::calculate_net_worth(&transactions.accounts)
    );
  }
}
