use chrono::{DateTime, Utc};
use colored::Colorize;

use crate::client::{SearchResponse, TagScope, TagValuesResponse};

pub fn format_trace(trace: &serde_json::Value, human_readable: bool) {
    if !human_readable {
        println!("{}", serde_json::to_string_pretty(trace).unwrap());
        return;
    }

    let batches = trace
        .get("batches")
        .or_else(|| trace.get("resourceSpans"))
        .and_then(|v| v.as_array());

    let Some(batches) = batches else {
        println!("{}", serde_json::to_string_pretty(trace).unwrap());
        return;
    };

    for batch in batches {
        let service_name = batch
            .pointer("/resource/attributes")
            .and_then(|attrs| attrs.as_array())
            .and_then(|attrs| {
                attrs
                    .iter()
                    .find(|a| a.get("key").and_then(|k| k.as_str()) == Some("service.name"))
            })
            .and_then(|a| a.pointer("/value/stringValue"))
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        println!("{}", format!("Service: {}", service_name).green().bold());

        let scope_spans = batch
            .get("scopeSpans")
            .or_else(|| batch.get("instrumentationLibrarySpans"))
            .and_then(|v| v.as_array());

        let Some(scope_spans) = scope_spans else {
            continue;
        };

        for scope in scope_spans {
            let Some(spans) = scope.get("spans").and_then(|v| v.as_array()) else {
                continue;
            };

            for span in spans {
                let name = span.get("name").and_then(|v| v.as_str()).unwrap_or("?");
                let kind = span
                    .get("kind")
                    .and_then(|v| v.as_u64())
                    .map(|k| match k {
                        1 => "INTERNAL",
                        2 => "SERVER",
                        3 => "CLIENT",
                        4 => "PRODUCER",
                        5 => "CONSUMER",
                        _ => "UNKNOWN",
                    })
                    .unwrap_or("?");

                let start_ns = span
                    .get("startTimeUnixNano")
                    .and_then(|v| {
                        v.as_str().or_else(|| v.as_u64().map(|_| "")).and_then(|s| {
                            if s.is_empty() {
                                v.as_u64().map(|n| n.to_string())
                            } else {
                                Some(s.to_string())
                            }
                        })
                    })
                    .and_then(|s| s.parse::<u64>().ok());

                let end_ns = span
                    .get("endTimeUnixNano")
                    .and_then(|v| {
                        v.as_str().or_else(|| v.as_u64().map(|_| "")).and_then(|s| {
                            if s.is_empty() {
                                v.as_u64().map(|n| n.to_string())
                            } else {
                                Some(s.to_string())
                            }
                        })
                    })
                    .and_then(|s| s.parse::<u64>().ok());

                let duration_str = match (start_ns, end_ns) {
                    (Some(s), Some(e)) if e > s => format_duration_ms((e - s) / 1_000_000),
                    _ => "?".to_string(),
                };

                let ts_str = start_ns
                    .and_then(|ns| {
                        DateTime::<Utc>::from_timestamp((ns / 1_000_000_000) as i64, (ns % 1_000_000_000) as u32)
                    })
                    .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                    .unwrap_or_default();

                let status = span
                    .pointer("/status/code")
                    .and_then(|v| v.as_u64())
                    .map(|c| match c {
                        0 => "UNSET",
                        1 => "OK",
                        2 => "ERROR",
                        _ => "?",
                    })
                    .unwrap_or("UNSET");

                let status_colored = match status {
                    "ERROR" => status.red().to_string(),
                    "OK" => status.green().to_string(),
                    _ => status.dimmed().to_string(),
                };

                println!(
                    "  {} {} {} {} {}",
                    name.cyan(),
                    kind.yellow(),
                    duration_str.bright_white(),
                    status_colored,
                    ts_str.dimmed()
                );
            }
        }
    }
}

pub fn format_search_results(data: &SearchResponse, human_readable: bool) {
    if !human_readable {
        println!("{}", serde_json::to_string_pretty(data).unwrap());
        return;
    }

    if data.traces.is_empty() {
        println!("{}", "No traces found.".dimmed());
        return;
    }

    for trace in &data.traces {
        let ts = trace
            .start_time_unix_nano
            .parse::<u64>()
            .ok()
            .and_then(|ns| DateTime::<Utc>::from_timestamp((ns / 1_000_000_000) as i64, (ns % 1_000_000_000) as u32))
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_default();

        println!(
            "{} {} {} {} {}",
            trace.trace_id.green().bold(),
            trace.root_service_name.cyan(),
            trace.root_trace_name.yellow(),
            format_duration_ms(trace.duration_ms).bright_white(),
            ts.dimmed()
        );
    }
}

pub fn format_tags(scopes: &[TagScope], human_readable: bool) {
    if !human_readable {
        println!("{}", serde_json::to_string_pretty(scopes).unwrap());
        return;
    }

    for scope in scopes {
        println!("{}", format!("[{}]", scope.name).green().bold());
        for tag in &scope.tags {
            println!("  {}", tag);
        }
    }
}

pub fn format_tag_values(data: &TagValuesResponse, human_readable: bool) {
    if !human_readable {
        println!("{}", serde_json::to_string_pretty(data).unwrap());
        return;
    }

    if data.tag_values.is_empty() {
        println!("{}", "No values found.".dimmed());
        return;
    }

    for tv in &data.tag_values {
        println!("{} {}", tv.value.bright_white(), tv.value_type.dimmed());
    }
}

fn format_duration_ms(ms: u64) -> String {
    if ms < 1000 {
        format!("{}ms", ms)
    } else if ms < 60_000 {
        format!("{:.1}s", ms as f64 / 1000.0)
    } else {
        format!("{:.1}m", ms as f64 / 60_000.0)
    }
}
