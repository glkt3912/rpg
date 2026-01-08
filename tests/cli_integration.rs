use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_basic_password_generation() {
    Command::cargo_bin("rpg")
        .unwrap()
        .assert()
        .success()
        .stdout(
            predicate::str::is_match(r"^[A-Za-z0-9!@#$%^&*()_+\-=\[\]{}|;:,.<>?]{16}\n$").unwrap(),
        );
}

#[test]
fn test_custom_length() {
    Command::cargo_bin("rpg")
        .unwrap()
        .args(&["-l", "20"])
        .assert()
        .success()
        .stdout(
            predicate::str::is_match(r"^[A-Za-z0-9!@#$%^&*()_+\-=\[\]{}|;:,.<>?]{20}\n$").unwrap(),
        );
}

#[test]
fn test_multiple_generation() {
    Command::cargo_bin("rpg")
        .unwrap()
        .args(&["-n", "3", "-l", "10"])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"(?m)^.{10,}$\n.{10,}$\n.{10,}$").unwrap());
}

#[test]
fn test_passphrase_generation() {
    Command::cargo_bin("rpg")
        .unwrap()
        .arg("--passphrase")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\w+-\w+-\w+-\w+\n$").unwrap());
}

#[test]
fn test_passphrase_custom_words() {
    Command::cargo_bin("rpg")
        .unwrap()
        .args(&["--passphrase", "--words", "6"])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^\w+-\w+-\w+-\w+-\w+-\w+\n$").unwrap());
}

#[test]
fn test_invalid_number_zero() {
    Command::cargo_bin("rpg")
        .unwrap()
        .args(&["-n", "0"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid generation count"));
}

#[test]
fn test_no_symbols_option() {
    Command::cargo_bin("rpg")
        .unwrap()
        .args(&["--no-symbols", "-l", "20"])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^[A-Za-z0-9]{20}\n$").unwrap());
}

#[test]
fn test_only_lowercase_and_digits() {
    Command::cargo_bin("rpg")
        .unwrap()
        .args(&["--no-uppercase", "--no-symbols", "-l", "15"])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^[a-z0-9]{15}\n$").unwrap());
}

#[test]
fn test_passphrase_and_multiple() {
    Command::cargo_bin("rpg")
        .unwrap()
        .args(&["--passphrase", "-n", "2"])
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"(?m)^\w+-\w+-\w+-\w+$\n\w+-\w+-\w+-\w+$").unwrap());
}

#[test]
fn test_help_option() {
    Command::cargo_bin("rpg")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage: rpg"))
        .stdout(predicate::str::contains("--passphrase"))
        .stdout(predicate::str::contains("--copy"));
}

#[test]
fn test_version_option() {
    Command::cargo_bin("rpg")
        .unwrap()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("rpg 0.2.0"));
}
