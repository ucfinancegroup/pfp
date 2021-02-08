#[allow(non_snake_case)]
pub mod TimeseriesService {
    use crate::common::{errors::ApiError, Money};
    use crate::controllers::timeseries_controller::{TimeseriesEntry, TimeseriesResponse};
    use crate::models::plan_model::Plan;
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

    pub fn generate_timeseries_from_plan(
        plan: Plan,
        days: i64,
        start_net_worth: Money,
        start_date: i64,
    ) -> Vec<TimeseriesEntry> {
        let mut date = Utc.timestamp(start_date, 0);
        let mut apy = 0;

        (1..days)
            .map(|d| {
                date = date + Duration::days(1);

                match plan
                    .allocations
                    .clone()
                    .into_iter()
                    .find(|a| a.date >= date.timestamp())
                {
                    Some(a) if a.date <= date.timestamp() => apy = 0, // recalculate apy
                    Some(_) => (),
                    None => (),
                };

                match plan
                    .events
                    .clone()
                    .into_iter()
                    .find(|a| a.start >= date.timestamp())
                {
                    Some(a) if a.start <= date.timestamp() => apy = 0, //recalculate apy
                    Some(_) => (),
                    None => (),
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
        let mut future: Vec<TimeseriesEntry>;
        let apy: f64 = 1.1; //temporary

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
