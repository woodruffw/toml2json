#![forbid(unsafe_code)]

use std::fs;
use std::io::{self, Read};

use anyhow::{Context, Result};
use clap::{App, Arg};

fn app<'a>() -> App<'a> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("pretty")
                .help("pretty print the JSON")
                .short('p')
                .long("pretty"),
        )
        .arg(
            Arg::new("input")
                .help("the TOML to convert")
                .index(1)
                .default_value("-"),
        )
}

fn main() -> Result<()> {
    let matches = app().get_matches();

    // Get our input source (which can be - or a filename) and its
    // corresponding buffer. We don't bother streaming or chunking,
    // since the `toml` crate only supports slices and strings.
    let input_src = matches.value_of("input").unwrap();
    let input_buf = match input_src {
        "-" => {
            let mut input_buf = String::new();
            io::stdin()
                .read_to_string(&mut input_buf)
                .with_context(|| "failed to collect stdin")?;
            input_buf
        }
        input => fs::read_to_string(input)
            .with_context(|| format!("failed to collect from input: {}", input))?,
    };

    // Turn our collected input into a value. We can't be more specific than
    // value, since we're doing arbitrary valid TOML conversions.
    let value = toml::from_str::<toml::Value>(&input_buf)
        .with_context(|| format!("parsing TOML from {} failed", input_src))?;

    // Spit back out, but as JSON. `serde_json` *does* support streaming, so
    // we do it.
    if matches.is_present("pretty") {
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
    fn test_app() {
        app().debug_assert();
    }
}
