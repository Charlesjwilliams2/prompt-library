// TODO(reader): Rust has no exceptions. Errors are values. As you read,
// track: which functions can fail, how do they say so in their signature,
// and who actually decides what to do about it?

use thiserror::Error;

// A library-style error type: specific, enumerable variants a caller can
// match on. This is what a well-behaved function-that-can-fail returns.
#[derive(Debug, Error)]
enum ConfigError {
    #[error("config line {line} is missing a '=' separator")]
    MissingSeparator { line: usize },
    #[error("value for '{key}' is not a valid number: {value}")]
    NotANumber { key: String, value: String },
}

#[derive(Debug)]
struct Config {
    max_retries: u32,
}

fn parse_config(input: &str) -> Result<Config, ConfigError> {
    let mut max_retries = 3; // default

    for (i, raw_line) in input.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }
        let (key, value) = line
            .split_once('=')
            .ok_or(ConfigError::MissingSeparator { line: i + 1 })?;
        let key = key.trim();
        let value = value.trim();

        if key == "max_retries" {
            max_retries = value.parse().map_err(|_| ConfigError::NotANumber {
                key: key.to_string(),
                value: value.to_string(),
            })?;
        }
    }

    Ok(Config { max_retries })
}

// `main`-level code usually doesn't care about *which* error variant it
// got — it just wants to report it and stop. `anyhow::Result` is the
// idiomatic choice at this boundary, and `?` still works because
// `ConfigError` implements `std::error::Error` (thanks to `thiserror`).
fn run() -> anyhow::Result<()> {
    // TODO(reader): this line has no `=` separator at all. Predict which
    // `ConfigError` variant `parse_config` returns for it before running.
    let missing_separator = "max_retries = 5\n# just a comment, no equals sign\n";

    let config = parse_config("max_retries = 5")?;
    println!("max_retries = {}", config.max_retries);

    match parse_config(missing_separator) {
        Ok(cfg) => println!("parsed anyway: max_retries = {}", cfg.max_retries),
        Err(e) => println!("expected failure: {e}"),
    }

    Ok(())
}

fn main() {
    // TODO(reader): what happens to the program if `run()` returns `Err`
    // here, versus if `main` used `.unwrap()` on every call inside `run`
    // instead of propagating with `?`?
    if let Err(e) = run() {
        eprintln!("fatal: {e:#}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_valid_line() {
        let config = parse_config("max_retries = 7").unwrap();
        assert_eq!(config.max_retries, 7);
    }

    #[test]
    fn missing_separator_is_an_error() {
        let err = parse_config("not a key value line").unwrap_err();
        assert!(matches!(err, ConfigError::MissingSeparator { line: 1 }));
    }

    #[test]
    fn non_numeric_value_is_an_error() {
        let err = parse_config("max_retries = many").unwrap_err();
        assert!(matches!(err, ConfigError::NotANumber { .. }));
    }
}
