use std::fs;
use assert_cmd::Command;

/// Test that if we pass no lines we will get back 0.
#[test]
fn test_part_1_empty_input() {
    Command::cargo_bin("day02_1")
        .unwrap()
        .assert()
        .success()
        .stdout(predicates::ord::eq("0\n"));
}

/// Test that if we pass example input we will generate expected output.
#[test]
fn test_part_1_example_input() {
    let input = concat!(
        "7 6 4 2 1\n",
        "1 2 7 8 9\n",
        "9 7 6 2 1\n",
        "1 3 2 4 5\n",
        "8 6 4 4 1\n",
        "1 3 6 7 9\n");

    Command::cargo_bin("day02_1")
        .unwrap()
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicates::ord::eq("2\n"));
}

/// Test that if we pass actual input we will compute expected output.
#[test]
fn test_part_1_actual_input() {
    let input: String = fs::read_to_string("inputs/2.txt").unwrap();

    Command::cargo_bin("day02_1")
        .unwrap()
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicates::ord::eq("407\n"));
}
