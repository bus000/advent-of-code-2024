// --- Part Two ---
//
// The engineers are surprised by the low number of safe reports until they
// realize they forgot to tell you about the Problem Dampener.
//
// The Problem Dampener is a reactor-mounted module that lets the reactor safety
// systems tolerate a single bad level in what would otherwise be a safe report.
// It's like the bad level never happened!
//
// Now, the same rules apply as before, except if removing a single level from
// an unsafe report would make it safe, the report instead counts as safe.
//
// More of the above example's reports are now safe:
//
// * 7 6 4 2 1: Safe without removing any level.
// * 1 2 7 8 9: Unsafe regardless of which level is removed.
// * 9 7 6 2 1: Unsafe regardless of which level is removed.
// * 1 3 2 4 5: Safe by removing the second level, 3.
// * 8 6 4 4 1: Safe by removing the third level, 4.
// * 1 3 6 7 9: Safe without removing any level.
//
// Thanks to the Problem Dampener, 4 reports are actually safe!
//
// Update your analysis by handling situations where the Problem Dampener can
// remove a single level from unsafe reports. How many reports are now safe?
use std::io;
use nom::{
    Parser,
    IResult,
    multi::{many1, separated_list1},
    character::complete::{one_of, char},
    combinator::{map_res, recognize, eof}
};

fn main() {
    let lines = io::stdin().lines();
    let mut safe = 0;
    for line in lines {
        let mut ns = parse_line(&line.unwrap());
        if is_safe(&mut ns) {
            safe = safe + 1;
        }
    }

    println!("{:?}", safe);
}

/// Check whether the report is safe.
///
/// Reports must be either monotonically increasing or decreasing. Reports must
/// only rise or fall by at most 3 at a time. Reports must either always rise or
/// fall. We are allowed to remove a single level from the report to make it
/// safe.
fn is_safe(report: &mut Vec<i32>) -> bool {
    if report.len() < 2 {
        return true;
    }
    if report[0] > report[report.len() - 1] {
        report.reverse();
    }
    let mut has_skipped = false;
    let mut last = report[0];

    for i in 1..report.len() {
        let current = report[i];
        let current_diff = current - last;
        if current_diff <= 0 || current_diff > 3 {
            // Check if we already failed.
            if has_skipped {
                return false;
            } else {
                has_skipped = true;
            }

            // If this is the last, then we are good.
            if i == report.len() - 1 {
                return true;
            }

            // Check if we can skip current.
            let next = report[i + 1];
            let next_diff = next - last;
            if next_diff > 0 && next_diff <= 3 {
                continue;
            }

            // If this is the first check we just skip the first number.
            if i == 1 {
                last = current;
                continue;
            }

            // Check if we can skip last.
            let before = report[i - 2];
            let before_diff = current - before;
            if before_diff > 0 && before_diff <= 3 {
                last = current;
                continue;
            }

            // We can neither skip current nor last.
            return false;
        } else {
            last = current;
        }
    }

    return true;
}

/// Parse a line from the input.
fn parse_line(input: &str) -> Vec<i32> {
    let (_, ns) = do_parse_line(input).unwrap();
    return ns;
}

/// Parse a line from the input.
fn do_parse_line(input: &str) -> IResult<&str, Vec<i32>> {
    let (input, ns) = separated_list1(char(' '), parse_i32)(input)?;
    let (input, _) = eof(input)?;
    return Ok((input, ns));
}

/// Parse an unsigned integer from the input.
///
/// Will return the parsed integer together with any remaining input.
fn parse_i32(input: &str) -> IResult<&str, i32> {
    map_res(recognize(many1(one_of("0123456789"))),
        |ns: &str| i32::from_str_radix(ns, 10)).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that parsing a number works as expected.
    #[test]
    fn test_parse_i32_valid_number() {
        let parsed = parse_i32("123whatever");
        assert_eq!(parsed, Ok(("whatever", 123)));
    }

    /// Test that empty numbers don't work.
    #[test]
    fn test_parse_i32_empty_number() {
        let parsed = parse_i32("whatever");
        assert!(parsed.is_err());
    }

    /// Test that parsing a line from the input works.
    #[test]
    fn test_parse_line() {
        let parsed = parse_line("1 2 3 4 5");
        assert_eq!(parsed, vec![1, 2, 3, 4, 5]);
    }

    /// Test that parsing a line from the input fails if line is not valid.
    #[test]
    #[should_panic]
    fn test_parse_line_invalid() {
        parse_line("3   4whatever");
    }

    /// Test that an safe report is reported as safe.
    #[test]
    fn test_is_safe() {
        assert!(is_safe(&mut vec![1, 3, 6, 7, 9]));
        assert!(is_safe(&mut vec![39, 37, 35, 32, 29, 27, 24]));
        assert!(is_safe(&mut vec![77, 79, 81, 82, 83, 85, 88]));
        assert!(is_safe(&mut vec![2, 5, 6, 7, 8, 12]));
        assert!(is_safe(&mut vec![2, 5, 4, 5, 7, 8]));
        assert!(is_safe(&mut vec![1, 3, 6, 7, 7, 9]));
        assert!(is_safe(&mut vec![1, 3, 2, 4, 5]));
        assert!(is_safe(&mut vec![1, 9, 10, 11, 13]));
        assert!(is_safe(&mut vec![4, 0]));
        assert!(is_safe(&mut vec![3, 0]));
        assert!(is_safe(&mut vec![1, 2]));
        assert!(is_safe(&mut vec![1]));
        assert!(is_safe(&mut vec![]));
        assert!(is_safe(&mut vec![43, 41, 45, 47, 49, 52, 55]));

        assert!(!is_safe(&mut vec![1, 2, 7, 8, 9]));
        assert!(!is_safe(&mut vec![66, 69, 70, 71, 73, 77, 78, 78]));
        assert!(!is_safe(&mut vec![77, 77, 79, 80, 79, 80]));
    }

}
