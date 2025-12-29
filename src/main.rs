#![forbid(unsafe_code)]

use std::fs;
use std::io::{self, Read};

use anyhow::{Context, Result};
use clap::Parser;

/// Convert TOML to JSON
#[derive(Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = env!("CARGO_PKG_DESCRIPTION"))]
struct Args {
    /// pretty print the JSON
    #[arg(short, long)]
    pretty: bool,

    /// the TOML to convert
    #[arg(default_value = "-")]
    input: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Get our input source (which can be - or a filename) and its
    // corresponding buffer. We don't bother streaming or chunking,
    // since the `toml` crate only supports slices and strings.
    let input_src = &args.input;
    let input_buf = match input_src.as_ref() {
        "-" => {
            let mut input_buf = String::new();
            io::stdin()
                .read_to_string(&mut input_buf)
                .with_context(|| "failed to collect stdin")?;
            input_buf
        }
        input => fs::read_to_string(input)
            .with_context(|| format!("failed to collect from input: {input}"))?,
    };

    // Turn our collected input into a value. We can't be more specific than
    // value, since we're doing arbitrary valid TOML conversions.
    let value = toml::from_str::<toml::Value>(&input_buf)
        .with_context(|| format!("parsing TOML from {input_src} failed"))?;

    // Spit back out, but as JSON. `serde_json` *does* support streaming, so
    // we do it.
    if args.pretty {
        serde_json::to_writer_pretty(io::stdout(), &value)
    } else {
        serde_json::to_writer(io::stdout(), &value)
    }
    .with_context(|| "JSON serialization and/or stdout streaming failed")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args() {
        use clap::CommandFactory;
        Args::command().debug_assert();
    }
}
