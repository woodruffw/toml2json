use std::io::Write;
use std::process::{Command, Output, Stdio};

fn run(args: &[&str], input: &[u8]) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_toml2json"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to start toml2json");
    child.stdin.take().unwrap().write_all(input).unwrap();
    child.wait_with_output().unwrap()
}

#[test]
fn stdin_output_ends_with_newline() {
    for (args, expected) in [
        (vec![], "{\"wow\":\"amazing\"}\n"),
        (vec!["-"], "{\"wow\":\"amazing\"}\n"),
        (vec!["--pretty"], "{\n  \"wow\": \"amazing\"\n}\n"),
        (vec!["--pretty", "-"], "{\n  \"wow\": \"amazing\"\n}\n"),
    ] {
        let output = run(&args, b"wow = \"amazing\"\n");
        assert!(output.status.success(), "{output:?}");
        assert_eq!(output.stdout, expected.as_bytes(), "args: {args:?}");
        assert!(output.stderr.is_empty(), "{output:?}");
    }
}

#[test]
fn file_output_ends_with_newline() {
    let input = concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml");
    for args in [vec![input], vec!["--pretty", input]] {
        let output = run(&args, b"");
        assert!(output.status.success(), "{output:?}");
        assert!(output.stdout.ends_with(b"\n"), "args: {args:?}");
        assert!(!output.stdout.ends_with(b"\n\n"), "args: {args:?}");
        let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(value["package"]["name"], "toml2json");
    }
}

#[test]
fn invalid_input_does_not_write_stdout() {
    for args in [vec![], vec!["--pretty"]] {
        let output = run(&args, b"wow = [");
        assert!(!output.status.success(), "{output:?}");
        assert!(output.stdout.is_empty(), "{output:?}");
        assert!(!output.stderr.is_empty(), "{output:?}");
    }
}

#[test]
fn closed_stdout_is_reported_as_error() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_toml2json"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to start toml2json");

    // Close the only reader before sending input so no output write can succeed.
    drop(child.stdout.take());
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"wow = \"amazing\"\n")
        .unwrap();
    let output = child.wait_with_output().unwrap();

    assert!(!output.status.success(), "{output:?}");
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("stdout"),
        "{output:?}"
    );
}
