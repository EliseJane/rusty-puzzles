extern crate chrono;
use chrono::{DateTime, Utc};
use std::time::Duration;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    start + Duration::to_std::seconds(1_000_000_000)
}
