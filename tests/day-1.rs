use assert_cmd::Command;

/// Test that if we pass no lines we will get back 0.
#[test]
fn test_empty_input() {
    Command::cargo_bin("advent-of-code-2024")
        .unwrap()
        .assert()
        .success()
        .stdout(predicates::ord::eq("0\n"));
}

/// Test that if we pass example input we will generate expected output.
#[test]
fn test_example_input() {
    let input = concat!(
        "3   4\n",
        "4   3\n",
        "2   5\n",
        "1   3\n",
        "3   9\n",
        "3   3\n"
    );

    Command::cargo_bin("advent-of-code-2024")
        .unwrap()
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicates::ord::eq("11\n"));
}
