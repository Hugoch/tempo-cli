use anyhow::Result;

use crate::client::TempoClient;
use crate::format::format_trace;

pub fn run(client: &TempoClient, trace_id: &str, human_readable: bool) -> Result<()> {
    let trace = client.get_trace(trace_id)?;
    format_trace(&trace, human_readable);
    Ok(())
}
