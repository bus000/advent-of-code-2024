use std::fs;
use assert_cmd::Command;

/// Test that if we pass example input we will generate expected output.
#[test]
fn test_part_1_example_input() {
    let input = concat![
        "....#.....\n",
        ".........#\n",
        "..........\n",
        "..#.......\n",
        ".......#..\n",
        "..........\n",
        ".#..^.....\n",
        "........#.\n",
        "#.........\n",
        "......#...\n"
    ];

    Command::cargo_bin("day06_1")
        .unwrap()
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicates::ord::eq("41\n"));
}

/// Test that if we pass actual input we will compute expected output.
#[test]
fn test_part_1_actual_input() {
    let input: String = fs::read_to_string("inputs/6.txt").unwrap();

    Command::cargo_bin("day06_1")
        .unwrap()
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicates::ord::eq("4665\n"));
}

/// Test that if we pass example input we will generate expected output.
#[test]
fn test_part_2_example_input() {
    let input = concat![
        "....#.....\n",
        ".........#\n",
        "..........\n",
        "..#.......\n",
        ".......#..\n",
        "..........\n",
        ".#..^.....\n",
        "........#.\n",
        "#.........\n",
        "......#...\n"
    ];

    Command::cargo_bin("day06_2")
        .unwrap()
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicates::ord::eq("6\n"));
}

/// Test that if we pass actual input we will compute expected output.
#[test]
fn test_part_2_actual_input() {
    let input: String = fs::read_to_string("inputs/6.txt").unwrap();

    Command::cargo_bin("day06_2")
        .unwrap()
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicates::ord::eq("1688\n"));
}
