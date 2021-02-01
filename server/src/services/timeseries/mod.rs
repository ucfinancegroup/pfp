#[allow(non_snake_case)]
pub mod TimeseriesService {
    use crate::controllers::timeseries_controller::TimeseriesEntry;
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

    //pub async fn get_timeseries() -> Result {}
}
