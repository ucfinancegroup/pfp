#[allow(non_snake_case)]
pub mod SnapshotService {
  use crate::common::{errors::ApiError, Money};
  use crate::models::user_model::{PlaidItem, Snapshot, User};
  use crate::services::finchplaid::ApiClient;
  use actix_web::web::Data;
  use chrono::{Duration, Utc};
  use log::debug;
  use plaid::models::{RetrieveTransactionsResponse, Transaction};
  use rust_decimal::Decimal;
  use std::collections::HashSet;
  use std::convert::TryFrom;

  pub async fn add_new_snapshot(
    user: &mut User,
    plaid_client: Data<ApiClient>,
  ) -> Result<(), ApiError> {
    let excluded_accounts = user.get_excluded_accounts();

    // handle each item connected to user
    let mut per_item_stats = Vec::new();
    for item in user.accounts.iter() {
      per_item_stats.push(handle_item(item, plaid_client.clone(), &excluded_accounts).await?);
    }

    // accumulate each item to a total
    let (total_money_in, total_money_out, total_net): (Money, Money, Money) =
      per_item_stats.iter().fold(
        (Money::default(), Money::default(), Money::default()),
        |(a, b, c), (e, f, g)| (a + *e, b + *f, c + *g),
      );

    // for rolling sums
    let last_snapshot = get_last_snapshot(&user.snapshots);

    // if user has no accounts,
    // calculated net worth will be zero
    // so we add in the user's self-reported net worth.
    let net_worth_adjustment = if user.accounts.is_empty() {
      user.net_worth
    } else {
      0.into()
    };

    // create the new snapshot
    let mut new_snapshot = Snapshot::new(
      total_net + net_worth_adjustment,
      total_money_in - total_money_out,
      total_money_out,
      total_money_in,
    );

    // patch so that we ignore recent stuff if the last snapshot was generated too recently.
    // https://github.com/ucfinancegroup/pfp/issues/212
    if chrono::DateTime::<Utc>::from_utc(
      chrono::NaiveDateTime::from_timestamp(last_snapshot.snapshot_time, 0),
      Utc,
    ) + chrono::Duration::days(1)
      > Utc::now()
    {
      log::debug!("Last snapshot too recent. Omitting last day of transactions");
      new_snapshot = Snapshot::new(
        total_net + net_worth_adjustment,
        0.into(),
        0.into(),
        0.into(),
      );
    }

    // make it a cumulative sum
    new_snapshot.running_savings.amount += last_snapshot.running_savings.amount;
    new_snapshot.running_spending.amount += last_snapshot.running_spending.amount;
    new_snapshot.running_income.amount += last_snapshot.running_income.amount;

    user.snapshots.push(new_snapshot);

    Ok(())
  }

  pub async fn handle_item(
    item: &PlaidItem,
    plaid_client: Data<ApiClient>,
    excluded_accounts: &HashSet<String>,
  ) -> Result<(Money, Money, Money), ApiError> {
    // accumulate money_in and money_out for items' transactions
    let (money_in, money_out) =
      get_money_in_out(item, plaid_client.clone(), excluded_accounts).await?;

    // get net worth of items accounts
    match crate::services::finchplaid::get_net_worth(item, plaid_client, excluded_accounts).await {
      Ok(num) => return Ok((money_in, money_out, num)),
      Err(e) => return Err(e),
    };
  }

  async fn get_money_in_out(
    item: &PlaidItem,
    plaid_client: Data<ApiClient>,
    excluded_accounts: &HashSet<String>,
  ) -> Result<(Money, Money), ApiError> {
    let transactions = get_item_transactions_for_new_snapshot(item, plaid_client).await?;

    Ok(calculate_money_in_out(&transactions, excluded_accounts))
  }

  pub fn calculate_money_in_out(
    transactions_response: &RetrieveTransactionsResponse,
    excluded_accounts: &HashSet<String>,
  ) -> (Money, Money) {
    // map each account to a coefficient for each transaction.
    let account_id_to_coeff = crate::services::finchplaid::get_account_transaction_coefficients(
      &transactions_response.accounts,
    );

    // accumulate money_in and money_out for transactions
    transactions_response
      .transactions
      .iter()
      .filter(|&transaction: &&Transaction| !excluded_accounts.contains(&transaction.account_id))
      .fold(
        (Money::default(), Money::default()).into(),
        |(money_in, money_out), transaction: &Transaction| {
          let s: Decimal = Decimal::try_from(transaction.amount)
            .and_then(|amount| {
              Ok(
                amount
                  * Decimal::new(
                    *account_id_to_coeff
                      .get(&transaction.account_id)
                      .or(Some(&0))
                      .unwrap(),
                    0,
                  ),
              )
            })
            .map_err(|e| {
              log::error!(
                "Could not convert {} to decimal: {}",
                transaction.amount.clone(),
                e
              );
            })
            .ok()
            .or(Some(Decimal::new(0, 0)))
            .unwrap();

          (money_in + s.max(0.into()), money_out + s.min(0.into()))
        },
      )
  }

  async fn get_item_transactions_for_new_snapshot(
    item: &PlaidItem,
    plaid_client: Data<ApiClient>,
  ) -> Result<RetrieveTransactionsResponse, ApiError> {
    // offset by 2 days to ensure we get a full day and avoid any timezone problems
    let date = (Utc::now() - chrono::Duration::days(2))
      .format("%Y-%m-%d")
      .to_string();

    // TODO: ensure we actually get all the transactions in a day. As of 2021 Jan 03,
    // the Plaid API returns up to 100 transactions by default. This should be enough
    // to cover one day of transactions for a normal person on one day.
    // ... but uhh maybe not thanks Robinhood.

    plaid_client
      .retrieve_transactions(item.access_token.clone(), date)
      .await
  }

  pub fn need_new_snapshot(snapshots: &Vec<Snapshot>) -> bool {
    let now = Utc::now().timestamp();
    let last_time = get_last_snapshot(snapshots).snapshot_time;

    debug!("Last snapshot at {}. Currently it is {}", last_time, now);

    // need a new snapshot if the last one was more than a day ago
    Duration::seconds(now - last_time) > Duration::days(1)
  }

  pub fn get_last_snapshot(snapshots: &Vec<Snapshot>) -> Snapshot {
    if snapshots.len() <= 0 {
      return Snapshot::default();
    }
    snapshots[snapshots.len() - 1].clone()
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::common::Money;
  use rust_decimal_macros::dec;
  use std::collections::HashSet;
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
      (Money::new(dec!(0)), Money::new(dec!(-10965.23))),
      SnapshotService::calculate_money_in_out(&transactions, &HashSet::new())
    );
  }

  #[test]
  fn test_calculate_net_worth() {
    let transactions = load_test_data().unwrap();
    assert_eq!(
      Money::new(dec!(68472.74)),
      crate::services::finchplaid::calculate_net_worth(&transactions.accounts, &HashSet::new())
    );

    // exclude plaid money market account and expect that the net worth should be lower, accordingly
    assert_eq!(
      Money::new(dec!(68472.74) - dec!(43200)),
      crate::services::finchplaid::calculate_net_worth(
        &transactions.accounts,
        &["jdgBn5mNDjSKwnLQng66C3n3mnRjMEi1mVMqx".to_string()]
          .iter()
          .cloned()
          .collect()
      )
    );
  }
}
