use std::process::Command;

#[test]
fn prints_the_welcome_message_with_the_crate_version() {
    let output = Command::new(env!("CARGO_BIN_EXE_cc-defines"))
        .output()
        .expect("failed to run binary");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        "Welcome to foo - version 1.0.2"
    );
}
