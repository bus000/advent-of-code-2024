use std::fs;
use assert_cmd::Command;

/// Test that if we pass no lines we will get back 0.
#[test]
fn test_part_1_empty_input() {
    Command::cargo_bin("day01_1")
        .unwrap()
        .assert()
        .success()
        .stdout(predicates::ord::eq("0\n"));
}

/// Test that if we pass example input we will generate expected output.
#[test]
fn test_part_1_example_input() {
    let input = concat!(
        "3   4\n",
        "4   3\n",
        "2   5\n",
        "1   3\n",
        "3   9\n",
        "3   3\n"
    );

    Command::cargo_bin("day01_1")
        .unwrap()
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicates::ord::eq("11\n"));
}

/// Test that if we pass actual input we will compute expected output.
#[test]
fn test_part_1_actual_input() {
    let input: String = fs::read_to_string("inputs/01.txt").unwrap();

    Command::cargo_bin("day01_1")
        .unwrap()
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicates::ord::eq("2086478\n"));
}

/// Test that if we pass no lines we will get back 0.
#[test]
fn test_part_2_empty_input() {
    Command::cargo_bin("day01_2")
        .unwrap()
        .assert()
        .success()
        .stdout(predicates::ord::eq("0\n"));
}

/// Test that if we pass example input we will generate expected output.
#[test]
fn test_part_2_example_input() {
    let input = concat!(
        "3   4\n",
        "4   3\n",
        "2   5\n",
        "1   3\n",
        "3   9\n",
        "3   3\n"
    );

    Command::cargo_bin("day01_2")
        .unwrap()
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicates::ord::eq("31\n"));
}

/// Test that if we pass actual input we will compute expected output.
#[test]
fn test_part_2_actual_input() {
    let input: String = fs::read_to_string("inputs/01.txt").unwrap();

    Command::cargo_bin("day01_2")
        .unwrap()
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicates::ord::eq("24941624\n"));
}
