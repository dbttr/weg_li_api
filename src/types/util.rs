use chrono::{DateTime, FixedOffset};

pub fn rfc3339_to_date_time(
    val: &String,
) -> Result<DateTime<FixedOffset>, chrono::format::ParseError> {
    match DateTime::parse_from_rfc3339(val) {
        Ok(val) => return Ok(val),
        Err(error) => return Err(error),
    }
}

pub fn date_time_to_rfc3339(val: &DateTime<FixedOffset>) -> String {
    val.to_rfc3339_opts(chrono::SecondsFormat::Millis, false)
}

const EXPORT_TIMESTAMP_FORMAT: &str = "%Y-%m-%d %H:%M:%S %.3f%z";

pub fn export_timestamp_to_date_time(
    val: &String,
) -> Result<DateTime<FixedOffset>, chrono::format::ParseError> {
    match DateTime::parse_from_str(val, EXPORT_TIMESTAMP_FORMAT) {
        Ok(val) => return Ok(val),
        Err(error) => return Err(error),
    }
}

pub fn date_time_to_export_timestamp(val: &DateTime<FixedOffset>) -> String {
    format!("{}", val.format(EXPORT_TIMESTAMP_FORMAT))
}
