use std::io::Write;
use std::process::{Command, Output, Stdio};

fn run(input: &str, args: &[&str]) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_toml2json"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    child.wait_with_output().unwrap()
}

#[test]
fn output_ends_with_newline() {
    for (args, expected) in [
        (&[][..], "{\"wow\":\"amazing\"}\n"),
        (&["-p"][..], "{\n  \"wow\": \"amazing\"\n}\n"),
    ] {
        let output = run("wow = \"amazing\"", args);
        assert!(output.status.success());
        assert_eq!(output.stdout, expected.as_bytes());
    }
}

#[test]
fn invalid_input_does_not_write_output() {
    let output = run("invalid TOML", &[]);
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
}
