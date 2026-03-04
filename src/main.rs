mod client;
mod commands;
mod format;
mod time;

use anyhow::Result;
use clap::{Parser, Subcommand};
use client::TempoClient;

#[derive(Parser)]
#[command(name = "tempo-cli")]
#[command(about = "CLI for querying Grafana Tempo traces")]
struct Cli {
    /// Human-readable colored output
    #[arg(short = 'H', long, global = true)]
    human: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get a trace by ID
    Get {
        /// Trace ID
        trace_id: String,
    },
    /// Search traces with TraceQL
    Search {
        /// TraceQL query string
        traceql: String,
        /// Start time (e.g., "1h", "now", RFC3339, Unix timestamp)
        #[arg(long)]
        start: Option<String>,
        /// End time
        #[arg(long)]
        end: Option<String>,
        /// Max number of traces to return
        #[arg(long)]
        limit: Option<u32>,
    },
    /// List available tag names
    Tags {
        /// Start time
        #[arg(long)]
        start: Option<String>,
        /// End time
        #[arg(long)]
        end: Option<String>,
    },
    /// List values for a tag
    TagValues {
        /// Tag name
        tag: String,
        /// Start time
        #[arg(long)]
        start: Option<String>,
        /// End time
        #[arg(long)]
        end: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let client = TempoClient::new()?;

    match cli.command {
        Commands::Get { trace_id } => {
            commands::get::run(&client, &trace_id, cli.human)?;
        },
        Commands::Search {
            traceql,
            start,
            end,
            limit,
        } => {
            commands::search::run(&client, &traceql, start.as_deref(), end.as_deref(), limit, cli.human)?;
        },
        Commands::Tags { start, end } => {
            commands::tags::run(&client, start.as_deref(), end.as_deref(), cli.human)?;
        },
        Commands::TagValues { tag, start, end } => {
            commands::tag_values::run(&client, &tag, start.as_deref(), end.as_deref(), cli.human)?;
        },
    }

    Ok(())
}
