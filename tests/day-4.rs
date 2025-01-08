use std::fs;
use assert_cmd::Command;

/// Test that if we pass no lines we will get back 0.
#[test]
fn test_part_1_empty_input() {
    Command::cargo_bin("day04_1")
        .unwrap()
        .assert()
        .success()
        .stdout(predicates::ord::eq("0\n"));
}

/// Test that if we pass example input we will generate expected output.
#[test]
fn test_part_1_example_input() {
    let input = concat![
        "MMMSXXMASM\n",
        "MSAMXMSMSA\n",
        "AMXSXMAAMM\n",
        "MSAMASMSMX\n",
        "XMASAMXAMM\n",
        "XXAMMXXAMA\n",
        "SMSMSASXSS\n",
        "SAXAMASAAA\n",
        "MAMMMXMMMM\n",
        "MXMXAXMASX"
    ];

    Command::cargo_bin("day04_1")
        .unwrap()
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicates::ord::eq("18\n"));
}

/// Test that if we pass actual input we will compute expected output.
#[test]
fn test_part_1_actual_input() {
    let input: String = fs::read_to_string("inputs/4.txt").unwrap();

    Command::cargo_bin("day04_1")
        .unwrap()
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicates::ord::eq("2551\n"));
}
