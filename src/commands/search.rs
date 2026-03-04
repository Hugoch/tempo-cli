use anyhow::Result;

use crate::client::TempoClient;
use crate::format::format_search_results;
use crate::time::parse_optional_time;

pub fn run(
    client: &TempoClient,
    query: &str,
    start: Option<&str>,
    end: Option<&str>,
    limit: Option<u32>,
    human_readable: bool,
) -> Result<()> {
    let start_ts = parse_optional_time(start)?;
    let end_ts = parse_optional_time(end)?;

    let data = client.search(query, start_ts.as_deref(), end_ts.as_deref(), limit)?;
    format_search_results(&data, human_readable);
    Ok(())
}
