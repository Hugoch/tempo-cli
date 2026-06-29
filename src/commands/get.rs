use anyhow::Result;

use crate::client::TempoClient;
use crate::format::format_trace;
use crate::time::parse_optional_time;

pub fn run(
    client: &TempoClient,
    trace_id: &str,
    start: Option<&str>,
    end: Option<&str>,
    human_readable: bool,
) -> Result<()> {
    let start_ts = parse_optional_time(start)?;
    let end_ts = parse_optional_time(end)?;

    let trace = client.get_trace(trace_id, start_ts.as_deref(), end_ts.as_deref())?;
    format_trace(&trace, human_readable);
    Ok(())
}
