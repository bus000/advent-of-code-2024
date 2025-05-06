use std::fs;
use assert_cmd::Command;

/// Test that if we pass example input we will generate expected output.
#[test]
fn test_part_1_example_input() {
    let input = concat![
        "RRRRIICCFF\n",
        "RRRRIICCCF\n",
        "VVRRRCCFFF\n",
        "VVRCCCJFFF\n",
        "VVVVCJJCFE\n",
        "VVIVCCJJEE\n",
        "VVIIICJJEE\n",
        "MIIIIIJJEE\n",
        "MIIISIJEEE\n",
        "MMMISSJEEE\n",
    ];

    Command::cargo_bin("day12_1")
        .unwrap()
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicates::ord::eq("1930\n"));
}

/// Test that if we pass actual input we will compute expected output.
#[test]
fn test_part_1_actual_input() {
    let input: String = fs::read_to_string("inputs/12.txt").unwrap();

    Command::cargo_bin("day12_1")
        .unwrap()
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicates::ord::eq("1433460\n"));
}
