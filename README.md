# tempo-cli

CLI for querying Grafana Tempo traces.

## Installation

```bash
cargo install --path .
```

## Configuration

| Environment Variable | Required | Description |
|---------------------|----------|-------------|
| `TEMPO_URL` | Yes | Base URL of the Tempo instance |
| `TEMPO_USER` | No | Basic auth username |
| `TEMPO_PASSWORD` | No | Basic auth password |

## Usage

```bash
# Get a trace by ID
tempo-cli get <traceID>
# Narrow the block-search window for large traces (avoids scanning all blocks)
tempo-cli get <traceID> --start 12h --end 9h

# Search traces with TraceQL
tempo-cli search '{ resource.service.name = "my-service" }'
tempo-cli search '{ status = error }' --start 1h --end now --limit 20

# List available tag names
tempo-cli tags
tempo-cli tags --start 1h --end now

# List values for a tag
tempo-cli tag-values service.name
```

### Global flags

| Flag | Description |
|------|-------------|
| `-H`, `--human` | Colored human-readable output (default: raw JSON) |

### Time format

The `--start` and `--end` flags accept:
- `now`
- Relative durations: `1h`, `30m`, `2d`
- RFC3339: `2024-01-01T00:00:00Z`
- Unix timestamps: `1704067200`

## Development

```bash
make fmt      # Format code (requires nightly)
make clippy   # Run clippy lints
```
