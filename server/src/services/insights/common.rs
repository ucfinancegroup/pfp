use crate::models::user_model::User;
use rust_decimal_macros::dec;
use wither::{
  mongodb::bson::{self, bson, doc, Bson, Document},
  Model,
};

pub fn match_income_range(u: &User) -> Document {
  doc! {
    "$match": {
      "income": {
        "$gte": bson::ser::to_bson(&(u.income * dec!(0.9))).unwrap(),
        "$lte": bson::ser::to_bson(&(u.income * dec!(1.1))).unwrap()
      },
      "snapshots": {
        "$not": {
          "$size": 0
        }
      },
      "_id": {
        "$ne": u.id().map_or_else(|| Bson::Null, |id| bson!(id))
      }
    }
  }
}
