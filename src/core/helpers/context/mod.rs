use chrono::{DateTime, Timelike, Utc};
use framework_cqrs_lib::cqrs::core::context::Context;
use framework_cqrs_lib::cqrs::models::errors::{Error, ResultErr};

pub fn give_date_time_with_hours(hour: u32, context: &Context) -> ResultErr<DateTime<Utc>> {
    context.now.with_hour(hour)
        .and_then(|d| d.with_minute(0))
        .and_then(|d| d.with_second(0))
        .ok_or(Error::Simple("Création date time impossible".to_string()))
}

pub fn ctx_is_after_datetime(datetime: &DateTime<Utc>, context: &Context) -> ResultErr<bool> {
    context
        .now
        .with_hour(context.now.hour() + 1u32)
        .ok_or(Error::Simple("Création date time impossible".to_string()))
        .map(|sanitize_date| sanitize_date < *datetime)
}
