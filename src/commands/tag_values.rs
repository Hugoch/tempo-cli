use anyhow::Result;

use crate::client::TempoClient;
use crate::format::format_tag_values;
use crate::time::parse_optional_time;

pub fn run(
    client: &TempoClient,
    tag: &str,
    start: Option<&str>,
    end: Option<&str>,
    human_readable: bool,
) -> Result<()> {
    let start_ts = parse_optional_time(start)?;
    let end_ts = parse_optional_time(end)?;

    let data = client.tag_values(tag, start_ts.as_deref(), end_ts.as_deref())?;
    format_tag_values(&data, human_readable);
    Ok(())
}
