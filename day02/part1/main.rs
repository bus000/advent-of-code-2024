// --- Day 2: Red-Nosed Reports ---
//
// Fortunately, the first location The Historians want to search isn't a long
// walk from the Chief Historian's office.
//
// While the Red-Nosed Reindeer nuclear fusion/fission plant appears to contain
// no sign of the Chief Historian, the engineers there run up to you as soon as
// they see you. Apparently, they still talk about the time Rudolph was saved
// through molecular synthesis from a single electron.
//
// They're quick to add that - since you're already here - they'd really
// appreciate your help analyzing some unusual data from the Red-Nosed reactor.
// You turn to check if The Historians are waiting for you, but they seem to
// have already divided into groups that are currently searching every corner of
// the facility. You offer to help with the unusual data.
//
// The unusual data (your puzzle input) consists of many reports, one report per
// line. Each report is a list of numbers called levels that are separated by
// spaces. For example:
//
//    7 6 4 2 1
//    1 2 7 8 9
//    9 7 6 2 1
//    1 3 2 4 5
//    8 6 4 4 1
//    1 3 6 7 9
//
// This example data contains six reports each containing five levels.
//
// The engineers are trying to figure out which reports are safe. The Red-Nosed
// reactor safety systems can only tolerate levels that are either gradually
// increasing or gradually decreasing. So, a report only counts as safe if both
// of the following are true:
//
// * The levels are either all increasing or all decreasing.
// * Any two adjacent levels differ by at least one and at most three.
//
// In the example above, the reports can be found safe or unsafe by checking
// those rules:
//
// * 7 6 4 2 1: Safe because the levels are all decreasing by 1 or 2.
// * 1 2 7 8 9: Unsafe because 2 7 is an increase of 5.
// * 9 7 6 2 1: Unsafe because 6 2 is a decrease of 4.
// * 1 3 2 4 5: Unsafe because 1 3 is increasing but 3 2 is decreasing.
// * 8 6 4 4 1: Unsafe because 4 4 is neither an increase or a decrease.
// * 1 3 6 7 9: Safe because the levels are all increasing by 1, 2, or 3.
//
// So, in this example, 2 reports are safe.
//
// Analyze the unusual data from the engineers. How many reports are safe?
use std::io;
use aoc2024::nom_helpers::parse_u32;
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
        let ns = parse_line(&line.unwrap());
        if is_safe(ns) {
            safe = safe + 1;
        }
    }

    println!("{:?}", safe);
}

/// Check whether the report is safe.
///
/// Reports must be either monotonically increasing or decreasing. Reports must
/// only rise or fall by at most 3 at a time. Reports must either always rise or
/// fall.
fn is_safe(report: Vec<u32>) -> bool {
    if report.len() <= 1 {
        return true;
    }

    let n1 = report[0];
    let n2 = report[1];
    let increase = n2 > n1;

    let mut last = n1;
    for next in report[1..].iter() {
        let difference = last.abs_diff(*next);
        if difference == 0 || difference > 3 {
            return false;
        } else if increase && next < &last {
            return false;
        } else if !increase && next > &last {
            return false;
        }

        last = *next;
    }

    return true;
}

/// Parse a line from the input.
fn parse_line(input: &str) -> Vec<u32> {
    let (_, ns) = do_parse_line(input).unwrap();
    return ns;
}

/// Parse a line from the input.
fn do_parse_line(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, ns) = separated_list1(char(' '), parse_u32)(input)?;
    let (input, _) = eof(input)?;
    return Ok((input, ns));
}

#[cfg(test)]
mod tests {
    use super::*;

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

    /// Test that empty reports are safe.
    #[test]
    fn test_is_safe_empty_report() {
        assert!(is_safe(vec![]));
    }

    /// Test that singleton reports are safe.
    #[test]
    fn test_is_safe_singleton_report() {
        assert!(is_safe(vec![1]));
    }

    /// Test that reports of 2 elements that are close together are safe.
    #[test]
    fn test_is_safe_2_elements_increasing_report() {
        assert!(is_safe(vec![1, 2]));
    }

    /// Test that reports of 2 elements that are close together are safe.
    #[test]
    fn test_is_safe_2_elements_decreasing_report() {
        assert!(is_safe(vec![3, 0]));
    }

    /// Test that reports of 2 elements that are too far away are not safe.
    #[test]
    fn test_is_safe_2_elements_too_far_report() {
        assert!(!is_safe(vec![4, 0]));
    }

    /// Test that an unsafe report is reported as unsafe.
    #[test]
    fn test_is_safe_unsafe() {
        assert!(!is_safe(vec![1, 2, 7, 8, 9]));
    }

    /// Test that an safe report is reported as safe.
    #[test]
    fn test_is_safe_safe() {
        assert!(is_safe(vec![1, 3, 6, 7, 9]));
    }

    /// Test that a report is unsafe if a number doesn't rise.
    #[test]
    fn test_is_safe_unsafe_if_no_change() {
        assert!(!is_safe(vec![1, 3, 6, 7, 7, 9]));
    }

}
