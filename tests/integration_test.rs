use assert_cmd::Command;

#[test]
fn test_cli_default() {
    let mut cmd = Command::cargo_bin("hello_trunk").unwrap();
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Hello, World!"));
}

#[test]
fn test_cli_with_name() {
    let mut cmd = Command::cargo_bin("hello_trunk").unwrap();
    cmd.arg("Bob")
        .assert()
        .success()
        .stdout(predicates::str::contains("Hello, Bob!"));
}