#[allow(non_snake_case)]
pub mod TimeseriesService {
    use crate::common::{errors::ApiError, Money};
    use crate::controllers::timeseries_controller::TimeseriesEntry;
    use crate::models::user_model::User;
    use chrono::{offset, Duration};
    use rust_decimal::Decimal;

    pub fn get_example() -> Vec<TimeseriesEntry> {
        let mut res = Vec::new();
        let mut today = offset::Utc::now();
        let mut start = today - Duration::weeks(54);
        let end = today + Duration::weeks(108);

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

        while today < end {
            res.push(TimeseriesEntry {
                date: today.timestamp(),
                net_worth: Decimal::new(last_value.clone(), 2).into(),
            });

            last_value += last_value * 3 / 1000;
            today = today + Duration::days(1);
        }

        return res;
    }

    pub async fn get_timeseries(user: User) -> Result<Vec<TimeseriesEntry>, ApiError> {
        let mut res = Vec::new();
        let today = offset::Utc::now();

        for item in user.snapshots.iter() {
            res.push(TimeseriesEntry {
                date: item.snapshot_time.clone(),
                net_worth: item.net_worth.clone(),
            });
        }

        Ok(res)
    }
}
