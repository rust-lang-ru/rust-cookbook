use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn greets_the_provided_name() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_cc-bundled-static"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to start binary");

    child
        .stdin
        .take()
        .expect("stdin not piped")
        .write_all(b"World\n")
        .expect("failed to write to stdin");

    let output = child.wait_with_output().expect("failed to wait on child");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("Hello from C!"));
    assert!(stdout.contains("Hello, World!"));
}
