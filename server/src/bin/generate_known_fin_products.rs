use pfp_server::models::financial_product_model::{FinancialProduct, FinancialProductInfo};
use serde_json::to_string_pretty;
use std::fs;

fn main() {
  let fin_products: Vec<FinancialProduct> = vec![
    FinancialProduct::new(
      "Plaid Gold Standard 0% Interest Checking",
      "Checking Account",
      vec![],
      None,
      None,
      FinancialProductInfo::Checking,
    ),
    FinancialProduct::new(
      "Plaid Silver Standard 0.1% Interest Saving",
      "Savings Account",
      vec![],
      None,
      None,
      FinancialProductInfo::Savings,
    ),
    FinancialProduct::new(
      "Plaid Bronze Standard 0.2% Interest CD",
      "Savings Account",
      vec![],
      None,
      None,
      FinancialProductInfo::Savings,
    ),
    FinancialProduct::new(
      "Plaid Diamond 12.5% APR Interest Credit Card",
      "Credit Card",
      vec![],
      None,
      None,
      FinancialProductInfo::CreditCard,
    ),
    FinancialProduct::new(
      "Plaid Platinum Standard 1.85% Interest Money Market",
      "Savings Account",
      vec![],
      None,
      None,
      FinancialProductInfo::Savings,
    ),
    FinancialProduct::new(
      "Plaid IRA",
      "Retirement Investment Account",
      vec![],
      None,
      None,
      FinancialProductInfo::Retirement,
    ),
    FinancialProduct::new(
      "Plaid 401k",
      "401k Retirement Account",
      vec![],
      None,
      None,
      FinancialProductInfo::Retirement,
    ),
    FinancialProduct::new(
      "Plaid Student Loan",
      "Student Loan",
      vec![],
      None,
      None,
      FinancialProductInfo::Loan,
    ),
    FinancialProduct::new(
      "Plaid Mortgage",
      "Mortgage",
      vec![],
      None,
      None,
      FinancialProductInfo::Loan,
    ),
  ];

  let s = to_string_pretty(&fin_products).unwrap();
  let _ = fs::write("file", s).unwrap();
}
