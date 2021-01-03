use crate::common::errors::ApiError;
use crate::models::user_model::{PlaidItem, Snapshot, User};
use crate::services::finchplaid::ApiClient;
use actix_web::web::Data;
use chrono::{Duration, Utc};
use futures::{
  future::join_all,
  stream::{self, StreamExt},
};
use plaid::models::{
  Account, RetrieveAnItemsAccountsRequest, RetrieveAnItemsAccountsResponse,
  RetrieveTransactionsRequest, RetrieveTransactionsResponse, Transaction,
};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct SnapshotService {}

impl SnapshotService {
  pub async fn new() -> SnapshotService {
    SnapshotService {}
  }

  pub async fn add_new_snapshot(
    user: &mut User,
    plaid_client: Data<Arc<Mutex<ApiClient>>>,
  ) -> Result<(), ApiError> {
    // handle each item connected to user

    let work = join_all(
      stream::iter(user.accounts.iter())
        .map(|item| Self::handle_item(item, plaid_client.clone()))
        .collect::<Vec<_>>()
        .await,
    )
    .await;

    let per_item_stats = work.into_iter().collect::<Result<Vec<_>, _>>()?;

    // accumulate each item to a total
    let (total_money_in, total_money_out, total_net) = per_item_stats
      .iter()
      .fold((0.0, 0.0, 0.0), |(a, b, c), (e, f, g)| {
        (a + e, b + f, c + g)
      });

    // for rolling sums
    let prev = Self::get_last_snapshot(&user.snapshots);

    // add the new snapshot
    user.snapshots.push(Snapshot {
      net_worth: total_net,
      running_savings: total_money_in - total_money_out + prev.running_savings,
      running_spending: total_money_out + prev.running_spending,
      running_income: total_money_in + prev.running_income,
      snapshot_time: Utc::now().timestamp(),
    });

    Ok(())
  }

  pub async fn handle_item(
    item: &PlaidItem,
    plaid_client: Data<Arc<Mutex<ApiClient>>>,
  ) -> Result<(f64, f64, f64), ApiError> {
    // accumulate money_in and money_out for items' transactions
    let (money_in, money_out) = Self::get_money_in_out(item, plaid_client.clone()).await?;

    // get net worth of items accounts
    let net_worth: f64 = Self::get_net_worth(item, plaid_client).await?;

    Ok((money_in, money_out, net_worth))
  }

  async fn get_money_in_out(
    item: &PlaidItem,
    plaid_client: Data<Arc<Mutex<ApiClient>>>,
  ) -> Result<(f64, f64), ApiError> {
    let transactions = Self::get_item_transactions_for_new_snapshot(item, plaid_client).await?;

    // map each account to a coefficient for each transaction.
    let account_id_to_coeff =
      crate::services::finchplaid::get_account_coefficients(&transactions.accounts);

    // accumulate money_in and money_out for transactions
    let (money_in, money_out): (f64, f64) = transactions.transactions.iter().fold(
      (0.0, 0.0),
      |(money_in, money_out), transaction: &Transaction| {
        let s: f64 = (transaction.amount as f64)
          * *account_id_to_coeff
            .get(&transaction.account_id)
            .or(Some(&0.0))
            .unwrap();
        (money_in + s.max(0.0), money_out + s.min(0.0))
      },
    );

    Ok((money_in, money_out))
  }

  async fn get_net_worth(
    item: &PlaidItem,
    plaid_client: Data<Arc<Mutex<ApiClient>>>,
  ) -> Result<f64, ApiError> {
    let accounts = Self::get_item_accounts_for_new_snapshot(item, plaid_client)
      .await?
      .accounts;

    // map each account to a coefficient for each transaction.
    let account_id_to_coeff = crate::services::finchplaid::get_account_coefficients(&accounts);

    //  calculate "net worth" of the item's accounts.
    let net_worth: f64 = accounts.iter().fold(0.0, |net, account: &Account| {
      let contribution: f64 = (account.balances.current as f64)
        * *account_id_to_coeff
          .get(&account.account_id)
          .or(Some(&0.0))
          .unwrap();
      net + contribution
    });

    Ok(net_worth)
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

  async fn get_item_accounts_for_new_snapshot(
    item: &PlaidItem,
    plaid_client: Data<Arc<Mutex<ApiClient>>>,
  ) -> Result<RetrieveAnItemsAccountsResponse, ApiError> {
    let pc = plaid_client.lock().unwrap();
    let config = &(pc.configuration);

    plaid::apis::item_management_api::retrieve_an_items_accounts(
      &config,
      RetrieveAnItemsAccountsRequest::new(
        pc.client_id.clone(),
        pc.secret.clone(),
        item.access_token.clone(),
      ),
    )
    .await
    .map_err(|_| ApiError::new(500, "Error while getting accounts".to_string()))
  }

  pub fn need_new_snapshot(snapshots: &Vec<Snapshot>) -> bool {
    let now = Utc::now().timestamp();
    let last_time = Self::get_last_snapshot(snapshots).snapshot_time;

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