#[allow(non_snake_case)]
pub mod TimeseriesService {
    use crate::common::{errors::ApiError, Money};
    use crate::controllers::timeseries_controller::{TimeseriesEntry, TimeseriesResponse};
    use crate::models::user_model::User;
    use chrono::{offset, Duration, TimeZone, Utc};
    use rust_decimal::Decimal;

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

    pub async fn get_timeseries(user: User, days: i64) -> Result<TimeseriesResponse, ApiError> {
        let mut res = Vec::new();
        let apy = 110; //temporary

        for item in user.snapshots.iter() {
            res.push(TimeseriesEntry {
                date: item.snapshot_time.clone(),
                net_worth: item.net_worth.clone(),
            });
        }

        // TODO: do something if user has no snapshots
        let last_day = user.snapshots[user.snapshots.len() - 1].clone();

        let next_day = Utc.timestamp(last_day.snapshot_time, 0);
        let mut account_value = last_day.net_worth;

        for i in 1..days {
            account_value = Money::from(Decimal::new(apy / 365, 2)) * account_value + account_value; //TODO: add transform stuff

            res.push(TimeseriesEntry {
                date: (next_day + Duration::days(i)).timestamp(),
                net_worth: account_value,
            });
        }

        Ok(TimeseriesResponse {
            start: last_day.snapshot_time,
            series: res,
        })
    }
}
