//! Key order survives the conversion (#406).
//!
//! JSON objects are unordered by specification, but a reader that got the order
//! from the TOML file cannot recover it later: a `toml2json | jsontoml` round
//! trip, or any `jq` transform of the result, reorders the whole file when the
//! order is lost. `preserve_order` on both `toml` and `serde_json` keeps it.

use std::io::Write;
use std::process::{Command, Stdio};

/// Alphabetical order is `alpha`, `middle`, `zeta`, so the input order below is
/// not the sorted one at any level. Every value is distinct so each key can be
/// located by its value alone.
const INPUT: &str = r#"
[zeta]
b = "b1"
a = "a1"

[middle]
x = "x1"

[alpha]
deep = { z = "z1", a = "a2" }
"#;

fn convert(pretty: bool) -> String {
    let mut args = vec![];
    if pretty {
        args.push("--pretty");
    }
    let mut child = Command::new(env!("CARGO_BIN_EXE_toml2json"))
        .args(&args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn toml2json");
    child
        .stdin
        .take()
        .expect("stdin is piped")
        .write_all(INPUT.as_bytes())
        .expect("write the TOML input");
    let output = child.wait_with_output().expect("wait for toml2json");
    assert!(output.status.success(), "toml2json failed");
    String::from_utf8(output.stdout).expect("stdout is UTF-8")
}

#[test]
fn tables_and_keys_follow_the_input_order() {
    for pretty in [false, true] {
        let json = convert(pretty);
        let at = |needle: &str| {
            json.find(needle)
                .unwrap_or_else(|| panic!("missing {needle} in {json}"))
        };
        assert!(at("\"zeta\"") < at("\"middle\""), "tables: {json}");
        assert!(at("\"middle\"") < at("\"alpha\""), "tables: {json}");
        assert!(at("\"b1\"") < at("\"a1\""), "table keys: {json}");
        assert!(at("\"z1\"") < at("\"a2\""), "inline table: {json}");
    }
}
