use anyhow::Result;

use crate::client::TempoClient;
use crate::format::format_tags;
use crate::time::parse_optional_time;

pub fn run(client: &TempoClient, start: Option<&str>, end: Option<&str>, human_readable: bool) -> Result<()> {
    let start_ts = parse_optional_time(start)?;
    let end_ts = parse_optional_time(end)?;

    let data = client.tags(start_ts.as_deref(), end_ts.as_deref())?;
    format_tags(&data.scopes, human_readable);
    Ok(())
}
