#[allow(non_snake_case)]
pub mod TimeseriesService {
    use crate::common::{errors::ApiError, Money};
    use crate::controllers::timeseries_controller::{TimeseriesEntry, TimeseriesResponse};
    use crate::models::plan_model::{Allocation, Event, Plan};
    use crate::models::recurring_model::Recurring;
    use crate::models::user_model::{Snapshot, User};
    use crate::services::finchplaid::ApiClient;
    use crate::services::users::UserService;
    use actix_web::web::Data;
    use chrono::{offset, Duration, TimeZone, Utc};
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;
    use std::sync::{Arc, Mutex};

    pub fn get_example() -> TimeseriesResponse {
        let mut res = Vec::new();
        let today = offset::Utc::now();
        let mut start = today - Duration::weeks(54);
        let end = today + Duration::weeks(108);
        let mut next_day = today.clone();

        let mut last_value: i64 = 100000;
        let mut i = 0;

        while start < today {
            res.push(TimeseriesEntry {
                date: start.timestamp(),
                net_worth: Decimal::new(last_value.clone(), 2).into(),
            });

            last_value += if (i % 3) == 0 {
                -321 * i - 2207
            } else {
                231 * i + 1408
            };
            start = start + Duration::days(1);
            i += 1;
        }

        while next_day < end {
            res.push(TimeseriesEntry {
                date: next_day.timestamp(),
                net_worth: Decimal::new(last_value.clone(), 2).into(),
            });

            last_value += last_value * 3 / 1000;
            next_day = next_day + Duration::days(1);
        }

        return TimeseriesResponse {
            start: today.timestamp(),
            series: res,
        };
    }

    pub fn generate_timeseries_from_snapshots(snapshots: Vec<Snapshot>) -> Vec<TimeseriesEntry> {
        snapshots
            .iter()
            .map(|s| TimeseriesEntry {
                date: s.snapshot_time.clone(),
                net_worth: s.net_worth.clone(),
            })
            .collect()
    }

    pub fn calculate_apy_from_allocation(allocation: Allocation) -> Decimal {
        allocation
            .schema
            .iter()
            .map(|a| a.change * a.asset.annualized_performance / dec!(100.0))
            .sum()
    }

    // for now only use static recurrings
    pub fn calculate_account_value(
        previous_value: Money,
        apy: Decimal,
        recurrings: Vec<Recurring>,
    ) -> Money {
        let recurring_value: Decimal = recurrings.iter().map(|r| r.amount).sum();
        return previous_value
            + previous_value * Money::from(apy / dec!(365.0))
            + Money::from(recurring_value);
    }

    pub fn generate_timeseries_from_plan(
        plan: Plan,
        days: i64,
        start_net_worth: Money,
        start_date: i64,
    ) -> Vec<TimeseriesEntry> {
        let start_date_dt = Utc.timestamp(start_date, 0);
        let mut apy = dec!(0.0);

        (1..days)
            .map(|d| start_date_dt + Duration::days(d))
            .map(|date| {
                apy = match plan
                    .allocations
                    .clone()
                    .into_iter()
                    .rev()
                    .find(|a| a.date <= date.timestamp())
                {
                    Some(a) => calculate_apy_from_allocation(a),
                    None => apy,
                };

                TimeseriesEntry {
                    date: date.timestamp(),
                    net_worth: start_net_worth.clone(),
                }
            })
            .collect()
    }

    pub async fn get_timeseries(
        mut user: User,
        days: i64,
        user_service: Data<UserService>,
        plaid_client: Data<Arc<Mutex<ApiClient>>>,
    ) -> Result<TimeseriesResponse, ApiError> {
        let mut past: Vec<TimeseriesEntry>;
        //let mut future: Vec<TimeseriesEntry>;

        let snapshots = user_service.get_snapshots(&mut user, plaid_client).await?;
        let last_day = snapshots[snapshots.len() - 1].clone();

        past = generate_timeseries_from_snapshots(snapshots);
        //future = generate_timeseries_from_plan(days, last_day.net_worth, last_day.snapshot_time);
        //past.append(&mut future);

        Ok(TimeseriesResponse {
            start: last_day.snapshot_time,
            series: past,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use chrono::{offset, DateTime, Duration, Utc};
    use rust_decimal_macros::dec;

    use crate::common::Money;
    use crate::controllers::timeseries_controller::{TimeseriesEntry, TimeseriesResponse};
    use crate::models::plan_model::{Allocation, AllocationChange, Asset};
    use crate::models::recurring_model::{Recurring, TimeInterval, Typ};
    use crate::models::user_model::Snapshot;
    use rust_decimal::Decimal;

    fn generate_snapshot_test_data(today: DateTime<Utc>) -> Vec<Snapshot> {
        (0..2)
            .map(|n| Snapshot {
                net_worth: Money::new(Decimal::new(n * 100, 0)),
                running_savings: Money::new(Decimal::new(n, 0)),
                running_spending: Money::new(Decimal::new(n, 0)),
                running_income: Money::new(Decimal::new(n, 0)),
                snapshot_time: (today - Duration::days(2 - n)).timestamp(),
            })
            .collect()
    }

    fn generate_snapshot_timeseries_verification(today: DateTime<Utc>) -> Vec<TimeseriesEntry> {
        (0..2)
            .map(|n| TimeseriesEntry {
                date: (today - Duration::days(2 - n)).timestamp(),
                net_worth: Money::new(Decimal::new(100 * n, 0)),
            })
            .collect()
    }

    #[test]
    fn test_snapshot_timeseries_generation() {
        let today = offset::Utc::now() - Duration::days(10);
        let generated = TimeseriesService::generate_timeseries_from_snapshots(
            generate_snapshot_test_data(today),
        );
        let verification = generate_snapshot_timeseries_verification(today);

        for i in (0..2) {
            assert_eq!(
                generated[i].net_worth == verification[i].net_worth
                    && generated[i].date == verification[i].date,
                true
            );
        }
    }

    #[test]
    fn test_allocation_apy_calculation_changed() {
        let test_asset = Asset {
            name: String::from("A Test Asset"),
            class: String::from("Stock"),
            annualized_performance: dec!(1.1),
        };

        let test_change = AllocationChange {
            asset: test_asset,
            change: dec!(100.0),
        };

        let test_allocation = Allocation {
            description: String::from("A Test Allocation"),
            date: offset::Utc::now().timestamp(),
            schema: vec![test_change],
        };

        let calculated_apy = TimeseriesService::calculate_apy_from_allocation(test_allocation);
        assert_eq!(calculated_apy, dec!(1.1));
    }

    #[test]
    fn test_allocation_apy_calculation_multiple_changed() {
        let test_asset1 = Asset {
            name: String::from("A Test Asset"),
            class: String::from("Stock"),
            annualized_performance: dec!(1.2),
        };

        let test_change1 = AllocationChange {
            asset: test_asset1,
            change: dec!(80.0),
        };

        let test_asset2 = Asset {
            name: String::from("A Test Asset"),
            class: String::from("Stock"),
            annualized_performance: dec!(0.7),
        };

        let test_change2 = AllocationChange {
            asset: test_asset2,
            change: dec!(20.0),
        };

        let test_allocation = Allocation {
            description: String::from("A Test Allocation"),
            date: offset::Utc::now().timestamp(),
            schema: vec![test_change1, test_change2],
        };

        let calculated_apy = TimeseriesService::calculate_apy_from_allocation(test_allocation);
        assert_eq!(calculated_apy, dec!(1.1));
    }

    #[test]
    fn test_account_value_calculation() {
        let test_apy = dec!(18.25);
        let initial_value = Money::from(dec!(100.0));
        let target_value = Money::from(dec!(205.0));

        let test_recurring = Recurring {
            id: None,
            name: String::from("Test Recurring"),
            start: (offset::Utc::now() - Duration::days(2)).timestamp(),
            end: (offset::Utc::now() + Duration::days(2)).timestamp(),
            principal: dec!(0.0),
            amount: dec!(100.0),
            interest: dec!(0.0),
            frequency: TimeInterval {
                typ: Typ::Monthly,
                content: 1,
            },
        };

        let calculated_value = TimeseriesService::calculate_account_value(
            initial_value,
            test_apy,
            vec![test_recurring],
        );
        assert_eq!(target_value, calculated_value);
    }

    #[test]
    fn test_account_value_calculation_negative_recurring() {
        let test_apy = dec!(18.25);
        let initial_value = Money::from(dec!(100.0));
        let target_value = Money::from(dec!(5.0));

        let test_recurring = Recurring {
            id: None,
            name: String::from("Test Recurring"),
            start: (offset::Utc::now() - Duration::days(2)).timestamp(),
            end: (offset::Utc::now() + Duration::days(2)).timestamp(),
            principal: dec!(0.0),
            amount: dec!(-100.0),
            interest: dec!(0.0),
            frequency: TimeInterval {
                typ: Typ::Monthly,
                content: 1,
            },
        };

        let calculated_value = TimeseriesService::calculate_account_value(
            initial_value,
            test_apy,
            vec![test_recurring],
        );
        assert_eq!(target_value, calculated_value);
    }

    #[test]
    fn test_account_value_calculation_from_allocation() {
        let test_asset1 = Asset {
            name: String::from("A Test Asset"),
            class: String::from("Stock"),
            annualized_performance: dec!(1.2),
        };

        let test_change1 = AllocationChange {
            asset: test_asset1,
            change: dec!(80.0),
        };

        let test_asset2 = Asset {
            name: String::from("A Test Asset"),
            class: String::from("Stock"),
            annualized_performance: dec!(0.7),
        };

        let test_change2 = AllocationChange {
            asset: test_asset2,
            change: dec!(20.0),
        };

        let test_allocation = Allocation {
            description: String::from("A Test Allocation"),
            date: offset::Utc::now().timestamp(),
            schema: vec![test_change1, test_change2],
        };

        let calculated_apy = TimeseriesService::calculate_apy_from_allocation(test_allocation);

        let initial_value = Money::from(dec!(100.0));
        let target_value = Money::from(dec!(200.30136986301369863013698630));

        let test_recurring = Recurring {
            id: None,
            name: String::from("Test Recurring"),
            start: (offset::Utc::now() - Duration::days(2)).timestamp(),
            end: (offset::Utc::now() + Duration::days(2)).timestamp(),
            principal: dec!(0.0),
            amount: dec!(100.0),
            interest: dec!(0.0),
            frequency: TimeInterval {
                typ: Typ::Monthly,
                content: 1,
            },
        };

        let calculated_value = TimeseriesService::calculate_account_value(
            initial_value,
            calculated_apy,
            vec![test_recurring],
        );
        assert_eq!(target_value, calculated_value);
    }
}
