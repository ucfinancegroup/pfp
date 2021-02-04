#[allow(non_snake_case)]
pub mod TimeseriesService {
    use crate::common::{errors::ApiError, Money};
    use crate::controllers::timeseries_controller::{TimeseriesEntry, TimeseriesResponse};
    use crate::models::user_model::User;
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

        past = snapshots
            .iter()
            .map(|s| TimeseriesEntry {
                date: s.snapshot_time.clone(),
                net_worth: s.net_worth.clone(),
            })
            .collect();

        // TODO: do something if user has no snapshots
        let last_day = user.snapshots[user.snapshots.len() - 1].clone();

        let next_day = Utc.timestamp(last_day.snapshot_time, 0);

        future = (1..days)
            .map(|s| TimeseriesEntry {
                date: (next_day + Duration::days(s)).timestamp(),
                net_worth: last_day.net_worth
                    * Money::from(Decimal::new(
                        ((apy / 365.0 + 1.0).powi(s as i32) * 10000.0) as i64,
                        4,
                    )),
            })
            .collect();
        past.append(&mut future);

        Ok(TimeseriesResponse {
            start: last_day.snapshot_time,
            series: past,
        })
    }
}
