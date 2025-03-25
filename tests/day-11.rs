use std::fs;
use assert_cmd::Command;

/// Test that if we pass example input we will generate expected output.
#[test]
fn test_part_1_example_input() {
    let input = "125 17\n";

    Command::cargo_bin("day11_1")
        .unwrap()
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicates::ord::eq("55312\n"));
}

/// Test that if we pass actual input we will compute expected output.
#[test]
fn test_part_1_actual_input() {
    let input: String = fs::read_to_string("inputs/11.txt").unwrap();

    Command::cargo_bin("day11_1")
        .unwrap()
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicates::ord::eq("229043\n"));
}
