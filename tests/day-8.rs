use std::fs;
use assert_cmd::Command;

/// Test that if we pass example input we will generate expected output.
#[test]
fn test_part_1_example_input() {
    let input = concat![
        "............\n",
        "........0...\n",
        ".....0......\n",
        ".......0....\n",
        "....0.......\n",
        "......A.....\n",
        "............\n",
        "............\n",
        "........A...\n",
        ".........A..\n",
        "............\n",
        "............\n"
    ];

    Command::cargo_bin("day08_1")
        .unwrap()
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicates::ord::eq("14\n"));
}

/// Test that if we pass actual input we will compute expected output.
#[test]
fn test_part_1_actual_input() {
    let input: String = fs::read_to_string("inputs/8.txt").unwrap();

    Command::cargo_bin("day08_1")
        .unwrap()
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicates::ord::eq("240\n"));
}

/// Test that if we pass example input we will generate expected output.
#[test]
fn test_part_2_example_input() {
    let input = concat![
        "............\n",
        "........0...\n",
        ".....0......\n",
        ".......0....\n",
        "....0.......\n",
        "......A.....\n",
        "............\n",
        "............\n",
        "........A...\n",
        ".........A..\n",
        "............\n",
        "............\n"
    ];

    Command::cargo_bin("day08_2")
        .unwrap()
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicates::ord::eq("34\n"));
}

/// Test that if we pass actual input we will compute expected output.
#[test]
fn test_part_2_actual_input() {
    let input: String = fs::read_to_string("inputs/8.txt").unwrap();

    Command::cargo_bin("day08_2")
        .unwrap()
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicates::ord::eq("955\n"));
}
