use anyhow::Result;
use chrono::Utc;

pub fn parse_time(input: &str) -> Result<String> {
    if input == "now" {
        return Ok(Utc::now().timestamp().to_string());
    }

    // Try parsing as duration (e.g., "1h", "30m")
    if let Ok(duration) = humantime::parse_duration(input) {
        let ts = Utc::now() - chrono::Duration::from_std(duration)?;
        return Ok(ts.timestamp().to_string());
    }

    // Try parsing as RFC3339
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(input) {
        return Ok(dt.timestamp().to_string());
    }

    // Try parsing as Unix timestamp
    if input.parse::<i64>().is_ok() {
        return Ok(input.to_string());
    }

    anyhow::bail!("Invalid time format: {}. Use 'now', duration (1h, 30m), RFC3339, or Unix timestamp", input)
}

pub fn parse_optional_time(input: Option<&str>) -> Result<Option<String>> {
    match input {
        Some(s) => Ok(Some(parse_time(s)?)),
        None => Ok(None),
    }
}
