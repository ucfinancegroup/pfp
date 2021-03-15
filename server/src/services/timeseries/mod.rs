#[allow(non_snake_case)]
pub mod TimeseriesService {
    use crate::common::{errors::ApiError, Money};
    use crate::controllers::timeseries_controller::{TimeseriesEntry, TimeseriesResponse};
    use crate::models::plan_model::{Allocation, AllocationProportion, Event, Plan};
    use crate::models::recurring_model::Recurring;
    use crate::models::user_model::{Snapshot, User};
    use crate::services::finchplaid::ApiClient;
    use crate::services::{plans::PlansService, users::UserService};
    use actix_web::web::Data;
    use chrono::{offset, DateTime, Duration, TimeZone, Utc};
    use rust_decimal::prelude::ToPrimitive;
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;

    struct RecurringState {
        rec: Recurring,
        skips: i32,
    }

    impl RecurringState {
        fn check_skip(&mut self) -> bool {
            self.skips += 1;
            self.skips % self.rec.frequency.content == 0
        }

        pub fn take_payment(&mut self, date: &DateTime<Utc>) -> Decimal {
            if !self.rec.is_active(date) || !self.check_skip() {
                return dec!(0);
            }

            if self.rec.principal == dec!(0) {
                self.rec.amount
            } else {
                self.rec.compound()
            }
        }
    }

    impl From<Recurring> for RecurringState {
        fn from(rec: Recurring) -> Self {
            Self { rec, skips: 0 }
        }
    }

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

    fn generate_timeseries_from_snapshots(snapshots: Vec<Snapshot>) -> Vec<TimeseriesEntry> {
        snapshots
            .iter()
            .map(|s| TimeseriesEntry {
                date: s.snapshot_time.clone(),
                net_worth: s.net_worth.clone(),
            })
            .collect()
    }

    fn calculate_apy_from_allocation(allocation: Allocation) -> Decimal {
        allocation
            .schema
            .iter()
            .map(|a| a.proportion * a.asset.annualized_performance / dec!(100.0))
            .sum()
    }

    fn calculate_account_value_from_apy(previous_value: Money, apy: Decimal) -> Money {
        let dpy = match apy.to_f64() {
            Some(p) => p.powf(1.0 / 365.0) * 1e9,
            None => 1e9, // if we cant convert, assume 1
        };

        return previous_value * Money::from(Decimal::new(dpy as i64, 9));
    }

    fn calculate_payments_from_recurrings(
        recurrings: &mut Vec<RecurringState>,
        date: &DateTime<Utc>,
    ) -> Money {
        let payments: Decimal = recurrings
            .iter_mut()
            .map(|rec| rec.take_payment(date))
            .sum();

        Money::from(payments)
    }

    fn calculate_account_value_from_event(event: Event, allocation: Allocation) -> Money {
        Money::from(
            allocation
                .schema
                .iter()
                .fold(dec!(0.0), |net, prop: &AllocationProportion| {
                    let part = event
                        .transforms
                        .iter()
                        .find(|change| change.class == prop.asset.class)
                        .cloned()
                        .map(|change| change.change * prop.proportion / dec!(10000.0))
                        .or(Some(prop.proportion))
                        .unwrap();
                    net + part
                }),
        )
    }

    pub fn generate_timeseries_from_plan(
        plan: Plan,
        days: i64,
        start_net_worth: Money,
        start_date: i64,
    ) -> Vec<TimeseriesEntry> {
        let start_date_dt = Utc.timestamp(start_date, 0);
        let mut apy = dec!(0.0);
        let mut net_worth = start_net_worth;

        let mut recurrings: Vec<RecurringState> = plan
            .recurrings
            .iter()
            .cloned()
            .map(RecurringState::from)
            .collect();

        let mut events = plan.events.clone();

        (1..days + 1)
            .map(|d| start_date_dt + Duration::days(d))
            .map(|date| {
                let allocation = plan
                    .allocations
                    .iter()
                    .rev()
                    .find(|a| a.date <= date.timestamp())
                    .cloned()
                    .unwrap();

                apy = calculate_apy_from_allocation(allocation.clone());

                net_worth = match events.iter().position(|a| a.start <= date.timestamp()) {
                    Some(i) => {
                        net_worth * calculate_account_value_from_event(events.remove(i), allocation)
                    }
                    None => net_worth,
                };

                let payments = calculate_payments_from_recurrings(&mut recurrings, &date);

                net_worth = calculate_account_value_from_apy(net_worth, apy) + payments;

                TimeseriesEntry {
                    date: date.timestamp(),
                    net_worth: net_worth.clone(),
                }
            })
            .collect()
    }

    pub async fn get_timeseries(
        mut user: User,
        days: i64,
        user_service: Data<UserService>,
        plaid_client: Data<ApiClient>,
    ) -> Result<TimeseriesResponse, ApiError> {
        let mut plan = if user.plans.len() > 0 {
            user.plans[0].clone()
        } else {
            PlansService::generate_sample_plan()
        };
        plan.recurrings.append(&mut user.recurrings.clone());

        let snapshots = user_service.get_snapshots(&mut user, plaid_client).await?;
        let last_day = snapshots[snapshots.len() - 1].clone();

        Ok(TimeseriesResponse {
            start: last_day.snapshot_time,
            series: generate_timeseries_from_snapshots(snapshots)
                .into_iter()
                .chain(generate_timeseries_from_plan(
                    plan,
                    days,
                    last_day.net_worth,
                    last_day.snapshot_time,
                ))
                .collect(),
        })
    }

    #[cfg(test)]
    mod test {
        use super::*;

        use chrono::{offset, DateTime, Duration, Utc};
        use rust_decimal_macros::dec;

        use crate::common::Money;
        use crate::controllers::timeseries_controller::TimeseriesEntry;
        use crate::models::plan_model::*;
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

        fn generate_plan_timeseries_verification(today: DateTime<Utc>) -> Vec<TimeseriesEntry> {
            (1..2)
                .map(|n| TimeseriesEntry {
                    date: (today + Duration::days(n)).timestamp(),
                    net_worth: Money::new(dec!(150.01305785)),
                })
                .collect()
        }

        fn generate_test_recurring() -> Recurring {
            Recurring {
                id: None,
                name: String::from("Test Recurring"),
                start: (offset::Utc::now() + Duration::days(1)).timestamp(),
                end: (offset::Utc::now() + Duration::days(2)).timestamp(),
                principal: dec!(0.0),
                amount: dec!(100.0),
                interest: dec!(0.0),
                frequency: TimeInterval {
                    typ: Typ::Monthly,
                    content: 1,
                },
            }
        }

        fn generate_test_allocation() -> Allocation {
            let test_asset1 = Asset {
                name: String::from("A Test Asset"),
                class: AssetClass::Equity,
                annualized_performance: dec!(1.2),
            };

            let test_change1 = AllocationProportion {
                asset: test_asset1,
                proportion: dec!(80.0),
            };

            let test_asset2 = Asset {
                name: String::from("A Test Asset"),
                class: AssetClass::Equity,
                annualized_performance: dec!(0.7),
            };

            let test_change2 = AllocationProportion {
                asset: test_asset2,
                proportion: dec!(20.0),
            };

            Allocation {
                id: None,
                description: String::from("A Test Allocation"),
                date: offset::Utc::now().timestamp(),
                schema: vec![test_change1, test_change2],
            }
        }

        #[test]
        fn test_snapshot_timeseries_generation() {
            let today = offset::Utc::now() - Duration::days(10);
            let generated = generate_timeseries_from_snapshots(generate_snapshot_test_data(today));
            let verification = generate_snapshot_timeseries_verification(today);

            for i in 0..2 {
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
                class: AssetClass::Equity,
                annualized_performance: dec!(1.1),
            };

            let test_change = AllocationProportion {
                asset: test_asset,
                proportion: dec!(100.0),
            };

            let test_allocation = Allocation {
                id: None,
                description: String::from("A Test Allocation"),
                date: offset::Utc::now().timestamp(),
                schema: vec![test_change],
            };

            let calculated_apy = calculate_apy_from_allocation(test_allocation);
            assert_eq!(calculated_apy, dec!(1.1));
        }

        #[test]
        fn test_allocation_apy_calculation_multiple_changed() {
            let test_allocation = generate_test_allocation();

            let calculated_apy = calculate_apy_from_allocation(test_allocation);
            assert_eq!(calculated_apy, dec!(1.1));
        }

        #[test]
        fn test_account_value_calculation() {
            let test_apy = dec!(1.1);
            let initial_value = Money::from(dec!(100.0));
            let target_value = Money::from(dec!(200.0261157000));

            let mut recurrings = vec![RecurringState::from(generate_test_recurring())];

            let calculated_value = calculate_account_value_from_apy(initial_value, test_apy)
                + calculate_payments_from_recurrings(
                    &mut recurrings,
                    &(Utc::now() + Duration::days(1)),
                );

            assert_eq!(target_value, calculated_value);
        }

        #[test]
        fn test_account_value_calculation_negative_recurring() {
            let test_apy = dec!(1.1);
            let initial_value = Money::from(dec!(100.0));
            let target_value = Money::from(dec!(0.0261157000));

            let mut recurrings = vec![RecurringState::from(Recurring {
                id: None,
                name: String::from("Test Recurring"),
                start: (offset::Utc::now() + Duration::days(1)).timestamp(),
                end: (offset::Utc::now() + Duration::days(2)).timestamp(),
                principal: dec!(0.0),
                amount: dec!(-100.0),
                interest: dec!(0.0),
                frequency: TimeInterval {
                    typ: Typ::Monthly,
                    content: 1,
                },
            })];

            let calculated_value = calculate_account_value_from_apy(initial_value, test_apy)
                + calculate_payments_from_recurrings(
                    &mut recurrings,
                    &(Utc::now() + Duration::days(1)),
                );
            assert_eq!(target_value, calculated_value);
        }

        #[test]
        fn test_account_value_calculation_from_allocation() {
            let test_allocation = generate_test_allocation();

            let calculated_apy = calculate_apy_from_allocation(test_allocation);

            let initial_value = Money::from(dec!(100.0));
            let target_value = Money::from(dec!(200.0261157000));

            let mut recurrings = vec![RecurringState::from(generate_test_recurring())];

            let calculated_value = calculate_account_value_from_apy(initial_value, calculated_apy)
                + calculate_payments_from_recurrings(
                    &mut recurrings,
                    &(Utc::now() + Duration::days(1)),
                );
            assert_eq!(target_value, calculated_value);
        }

        #[test]
        fn test_compounding_recurring() {
            let mut recurrings = vec![RecurringState::from(Recurring {
                id: None,
                name: String::from("Test Compounding Recurring"),
                start: (offset::Utc::now()).timestamp(),
                end: (offset::Utc::now() + Duration::days(2)).timestamp(),
                principal: dec!(100.0),
                interest: dec!(12),
                amount: dec!(0),
                frequency: TimeInterval {
                    typ: Typ::Daily,
                    content: 1,
                },
            })];

            let day_one = calculate_payments_from_recurrings(&mut recurrings, &Utc::now());
            let day_two = calculate_payments_from_recurrings(
                &mut recurrings,
                &(Utc::now() + Duration::days(1)),
            );

            assert_eq!(Money::from(dec!(12) + dec!(13.44)), day_one + day_two);
        }

        #[test]
        fn test_generate_timeseries_from_plan() {
            let start_net_worth = Money::from(dec!(100.0));
            let days = 1;
            let start_date = offset::Utc::now();

            let test_recurrings = vec![generate_test_recurring()];

            let test_allocations = vec![generate_test_allocation()];

            let test_events = vec![Event {
                id: None,
                name: String::from("Test Event"),
                start: start_date.timestamp(),
                transforms: vec![AssetClassChange {
                    class: AssetClass::Equity,
                    change: dec!(50.0),
                }],
            }];

            let test_plan = Plan {
                id: None,
                name: String::from("Test Plan"),
                recurrings: test_recurrings,
                allocations: test_allocations,
                events: test_events,
            };

            let generated = generate_timeseries_from_plan(
                test_plan,
                days,
                start_net_worth,
                start_date.timestamp(),
            );

            let verification = generate_plan_timeseries_verification(start_date);

            for i in 0..1 {
                assert_eq!(generated[i], verification[i]);
            }
        }
    }
}
