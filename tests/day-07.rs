use std::fs;
use assert_cmd::Command;

/// Test that if we pass example input we will generate expected output.
#[test]
fn test_part_1_example_input() {
    let input = concat![
        "190: 10 19\n",
        "3267: 81 40 27\n",
        "83: 17 5\n",
        "156: 15 6\n",
        "7290: 6 8 6 15\n",
        "161011: 16 10 13\n",
        "192: 17 8 14\n",
        "21037: 9 7 18 13\n",
        "292: 11 6 16 20\n"
    ];

    Command::cargo_bin("day07_1")
        .unwrap()
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicates::ord::eq("3749\n"));
}

/// Test that if we pass actual input we will compute expected output.
#[test]
fn test_part_1_actual_input() {
    let input: String = fs::read_to_string("inputs/07.txt").unwrap();

    Command::cargo_bin("day07_1")
        .unwrap()
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicates::ord::eq("4555081946288\n"));
}

/// Test that if we pass example input we will generate expected output.
#[test]
fn test_part_2_example_input() {
    let input = concat![
        "190: 10 19\n",
        "3267: 81 40 27\n",
        "83: 17 5\n",
        "156: 15 6\n",
        "7290: 6 8 6 15\n",
        "161011: 16 10 13\n",
        "192: 17 8 14\n",
        "21037: 9 7 18 13\n",
        "292: 11 6 16 20\n"
    ];

    Command::cargo_bin("day07_2")
        .unwrap()
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicates::ord::eq("11387\n"));
}

/// Test that if we pass actual input we will compute expected output.
#[test]
fn test_part_2_actual_input() {
    let input: String = fs::read_to_string("inputs/07.txt").unwrap();

    Command::cargo_bin("day07_2")
        .unwrap()
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicates::ord::eq("227921760109726\n"));
}
