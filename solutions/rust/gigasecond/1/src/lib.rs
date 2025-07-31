extern crate chrono;
use chrono::*;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let after_timepoint: i64 = start.timestamp() + 1_000_000_000;
    let after_naive_datetime: NaiveDateTime = NaiveDateTime::from_timestamp(after_timepoint, 0);
    let after_utc_datetime: DateTime<Utc> =  DateTime::<Utc>::from_utc(after_naive_datetime, Utc);
    return after_utc_datetime;
}