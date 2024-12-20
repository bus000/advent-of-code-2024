use std::fs;
use assert_cmd::Command;

/// Test that if we pass no lines we will get back 0.
#[test]
fn test_part_1_empty_input() {
    Command::cargo_bin("day03_1")
        .unwrap()
        .assert()
        .success()
        .stdout(predicates::ord::eq("0\n"));
}

/// Test that if we pass example input we will generate expected output.
#[test]
fn test_part_1_example_input() {
    let input =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    Command::cargo_bin("day03_1")
        .unwrap()
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicates::ord::eq("161\n"));
}

/// Test that if we pass actual input we will compute expected output.
#[test]
fn test_part_1_actual_input() {
    let input: String = fs::read_to_string("inputs/3.txt").unwrap();

    Command::cargo_bin("day03_1")
        .unwrap()
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicates::ord::eq("184511516\n"));
}
