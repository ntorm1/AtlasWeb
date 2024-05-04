use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};

pub fn string_to_nanoseconds_epoch(
    input_string: &str,
    format_string: &str,
) -> Result<i64, chrono::format::ParseError> {
    if let Ok(dt) = DateTime::parse_from_str(input_string, format_string) {
        return Ok(dt.with_timezone(&Utc).timestamp());
    } else {
        let date = NaiveDate::parse_from_str(input_string, format_string)?;
        let dt = NaiveDateTime::new(date, NaiveTime::from_hms_opt(0, 0, 0).unwrap());
        let datetime: DateTime<Utc> = DateTime::from_utc(dt, Utc);
        return Ok(datetime.timestamp());
    }
}
