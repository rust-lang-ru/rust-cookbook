use std::process::Command;

#[test]
fn multiplies_the_two_operands() {
    let output = Command::new(env!("CARGO_BIN_EXE_cc-bundled-cpp"))
        .output()
        .expect("failed to run binary");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "35");
}
