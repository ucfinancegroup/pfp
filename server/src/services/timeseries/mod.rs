#[allow(non_snake_case)]
pub mod TimeseriesService {
    use crate::common::{errors::ApiError, Money};
    use crate::controllers::timeseries_controller::{TimeseriesEntry, TimeseriesResponse};
    use crate::models::plan_model::{Allocation, Event, Plan};
    use crate::models::user_model::{Snapshot, User};
    use crate::services::finchplaid::ApiClient;
    use crate::services::users::UserService;
    use actix_web::web::Data;
    use chrono::{offset, Duration, TimeZone, Utc};
    use rust_decimal::Decimal;
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

    pub fn calculate_apy_from_allocation(
        allocation: Allocation,
        current_apy: f64,
        date: i64,
    ) -> f64 {
        if date >= allocation.date {
            return current_apy + 0.1;
        }
        current_apy
    }

    pub fn generate_timeseries_from_plan(
        plan: Plan,
        days: i64,
        start_net_worth: Money,
        start_date: i64,
    ) -> Vec<TimeseriesEntry> {
        let mut date = Utc.timestamp(start_date, 0);
        let mut apy: f64 = 0.0;

        (1..days)
            .map(|_d| {
                date = date + Duration::days(1);

                match plan
                    .allocations
                    .clone()
                    .into_iter()
                    .find(|a| a.date >= date.timestamp())
                {
                    Some(a) => apy = calculate_apy_from_allocation(a, apy, date.timestamp()),
                    None => (),
                };

                /* idk how to incorporate events into apy calculations yet
                match plan
                    .events
                    .clone()
                    .into_iter()
                    .find(|a| a.start >= date.timestamp())
                {
                    Some(a) => (),
                    None => (),
                };*/

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

        let mut values_equal = true;

        for i in (0..2) {
            if generated[i].net_worth != verification[i].net_worth {
                values_equal = false;
            }

            if generated[i].date != verification[i].date {
                values_equal = false;
            }
        }

        assert_eq!(values_equal, true);
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

        let calculated_apy = TimeseriesService::calculate_apy_from_allocation(
            test_allocation,
            1.0,
            offset::Utc::now().timestamp(),
        );
        assert_eq!(calculated_apy, 1.1);
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

        let calculated_apy = TimeseriesService::calculate_apy_from_allocation(
            test_allocation,
            1.0,
            offset::Utc::now().timestamp(),
        );
        assert_eq!(calculated_apy, 1.1);
    }

    #[test]
    fn test_allocation_apy_calculation_unchanged() {
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

        let calculated_apy = TimeseriesService::calculate_apy_from_allocation(
            test_allocation,
            1.0,
            (offset::Utc::now() - Duration::days(2)).timestamp(),
        );
        assert_eq!(calculated_apy, 1.0);
    }
}
