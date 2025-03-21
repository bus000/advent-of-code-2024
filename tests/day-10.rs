use std::fs;
use assert_cmd::Command;

/// Test that if we pass example input we will generate expected output.
#[test]
fn test_part_1_example_input() {
    let input = concat![
        "89010123\n",
        "78121874\n",
        "87430965\n",
        "96549874\n",
        "45678903\n",
        "32019012\n",
        "01329801\n",
        "10456732\n",
    ];

    Command::cargo_bin("day10_1")
        .unwrap()
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicates::ord::eq("36\n"));
}

/// Test that if we pass actual input we will compute expected output.
#[test]
fn test_part_1_actual_input() {
    let input: String = fs::read_to_string("inputs/10.txt").unwrap();

    Command::cargo_bin("day10_1")
        .unwrap()
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicates::ord::eq("667\n"));
}
