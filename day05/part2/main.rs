// --- Part Two ---
//
// While the Elves get to work printing the correctly-ordered updates, you have
// a little time to fix the rest of them.
//
// For each of the incorrectly-ordered updates, use the page ordering rules to
// put the page numbers in the right order. For the above example, here are the
// three incorrectly-ordered updates and their correct orderings:
//
//  * 75,97,47,61,53 becomes 97,75,47,61,53.
//  * 61,13,29 becomes 61,29,13.
//  * 97,13,75,29,47 becomes 97,75,47,29,13.
//
// After taking only the incorrectly-ordered updates and ordering them
// correctly, their middle page numbers are 47, 29, and 47. Adding these
// together produces 123.
//
// Find the updates which are not in the correct order. What do you get if you
// add up the middle page numbers after correctly ordering just those updates?
use std::io;
use std::io::Read;
use std::collections::HashMap;
use std::fmt::Debug;
use nom::IResult;
use nom::multi::{many0, separated_list0, separated_list1};
use nom::character::complete::char;
use nom::combinator::all_consuming;
use aoc2024::nom_helpers::parse_u32;
use aoc2024::aoc::AocError;

fn main() -> Result<(), AocError> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let input = parse_input(&input)?;
    let mut sum = 0;
    for update in &input.updates {
        let fixed = fix_update(&input.rules, update);
        if fixed != *update {
            let middle = middle(&fixed)
                .ok_or(AocError::UnexpectedError(
                        "Expected middle.".to_string()))?;
            sum = sum + middle;
        }
    }
    println!("{:?}", sum);
    return Ok(());
}

/// A page is represented just by its page number.
type Page = u32;

/// A rule specifies that a particular page must come before another page.
type Rule = (Page, Page);

/// An update is a list of pages to update.
type Update = Vec<Page>;

/// Parsed input.
#[derive(Debug, Eq, PartialEq)]
struct Input {

    /// List of rules the updates must follow.
    rules: Vec<Rule>,

    /// List of updates which consist of lists of pages.
    updates: Vec<Update>

}

/// Fix the given update by swapping around numbers such that the update
/// conforms to all the given rules.
fn fix_update(rules: &Vec<Rule>, update: &Vec<Page>) -> Vec<Page> {
    let mut indices: HashMap<Page, usize> = update
        .iter()
        .enumerate()
        .map(|(i, page)| (*page, i))
        .collect();

    loop {
        let mut changed = false;
        for rule in rules {
            changed = apply_rule(rule, &mut indices) || changed;
        }

        if !changed {
            let mut fixed = vec![0; update.len()];
            for (page, i) in indices {
                fixed[i] = page;
            }
            return fixed;
        }
    }
}

/// Apply the given rule by swapping the position of the before number with the
/// after number if the before index is greater than the after index.
fn apply_rule(rule: &Rule, indices: &mut HashMap<u32, usize>) -> bool {
    let (before, after) = rule;
    let b_index = indices.get(before).map(|x| *x);
    let a_index = indices.get(after).map(|x| *x);
    match (b_index, a_index) {
        (Some(b_index), Some(a_index)) => {
            if b_index > a_index {
                indices.insert(*before, a_index);
                indices.insert(*after, b_index);
                return true;
            }
        }
        _ => {}
    }

    return false;
}

/// Find the middle element of a vector.
///
/// If the vector has an even number of elements, it has no middle and None will
/// be returned.
fn middle<T>(list: &Vec<T>) -> Option<&T> {
    let length = list.len();
    if length % 2 == 0 {
        return None;
    } else {
        return Some(&list[length / 2]);
    }
}

/// Parse an Input from the given input string.
fn parse_input(input: &str) -> Result<Input, AocError> {
    let (_, parsed) = all_consuming(do_parse_input)(input)?;
    return Ok(parsed);
}

/// Parse an Input from the given input string.
fn do_parse_input(input: &str) -> IResult<&str, Input> {
    let (input, rules) = parse_rules(input)?;
    let (input, _) = many0(char('\n'))(input)?;
    let (input, updates) = parse_updates(input)?;
    let (input, _) = char('\n')(input)?;

    return Ok((input, Input {
        rules: rules,
        updates: updates
    }));
}

/// Parse a list of rules separated by newlines.
fn parse_rules(input: &str) -> IResult<&str, Vec<(Page, Page)>> {
    separated_list0(char('\n'), parse_rule)(input)
}

/// Parses a single rule from the input or errors.
///
/// A rule is a number followed by | followed by another number, like:
///
///   123|321
fn parse_rule(input: &str) -> IResult<&str, (Page, Page)> {
    let (input, n1) = parse_u32(input)?;
    let (input, _) = char('|')(input)?;
    let (input, n2) = parse_u32(input)?;

    return Ok((input, (n1, n2)));
}

/// Parses a list of updates separated by newlines.
fn parse_updates(input: &str) -> IResult<&str, Vec<Update>> {
    separated_list0(char('\n'), parse_update)(input)
}

/// Parse an update which is a list of numbers separated by commas.
fn parse_update(input: &str) -> IResult<&str, Update> {
    separated_list1(char(','), parse_u32)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that parsing an update works.
    #[test]
    fn test_parse_update() {
        assert_eq!(Ok(("", vec![1, 2, 3])), parse_update("1,2,3"));
        assert_eq!(Ok(("\n3,2", vec![1, 2, 3])), parse_update("1,2,3\n3,2"));
        assert_eq!(Ok(("\n3,2", vec![3])), parse_update("3\n3,2"));
        let error = parse_update("");
        assert!(error.is_err());
        let error = parse_update("whatever");
        assert!(error.is_err());
    }

    /// Test that parsing multiple updates work.
    #[test]
    fn test_parse_updates_1() {
        let input = concat![
            "1,2,3\n",
            "4,5,6\n"
        ];
        let expected = vec![
            vec![1, 2, 3],
            vec![4, 5, 6]
        ];
        assert_eq!(Ok(("\n", expected)), parse_updates(input));
    }

    /// Test that parsing multiple updates work.
    #[test]
    fn test_parse_updates_2() {
        let input = concat![
            "\n1,2,3\n",
            "4,5,6\n"
        ];
        let expected = vec![];
        assert_eq!(Ok((input, expected)), parse_updates(input));
    }

    /// Test that parsing a rule works as expected.
    #[test]
    fn test_parse_rule() {
        assert_eq!(Ok(("", (1, 2))), parse_rule("1|2"));
        assert_eq!(Ok(("\n", (1, 2))), parse_rule("1|2\n"));
        let error = parse_rule("whatever");
        assert!(error.is_err());
    }

    /// Test that parsing rules works as expected.
    #[test]
    fn test_parse_rules() {
        assert_eq!(Ok(("", vec![(1, 2)])), parse_rules("1|2"));
        assert_eq!(Ok(("\n", vec![(1, 2), (3, 5)])), parse_rules("1|2\n3|5\n"));
        assert_eq!(Ok(("whatever", vec![])), parse_rules("whatever"));
    }

    /// Test that parsing input works as expected.
    #[test]
    fn test_parse_input_1() {
        let input = concat![
            "1|2\n",
            "3|4\n",
            "\n",
            "1,2,3\n",
            "4,5,6\n"
        ];
        let expected = Input {
            rules: vec![(1, 2), (3, 4)],
            updates: vec![
                vec![1, 2, 3],
                vec![4, 5, 6]
            ]
        };
        assert_eq!(Ok(expected), parse_input(input));
    }

    /// Test that parsing input works as expected.
    #[test]
    fn test_parse_input_2() {
        let input = concat![
            "1|2\n",
            "3|4\n",
            "\n",
            "1,2,3\n",
            "4,5,6\nwhatever\n"
        ];
        let error = parse_input(input);
        assert!(error.is_err());
    }

    /// Test that getting the middle of a list works.
    #[test]
    fn test_middle() {
        assert_eq!(Some(&4), middle(&vec![1, 2, 3, 4, 3, 2, 1]));
        assert_eq!(Some(&3), middle(&vec![1, 2, 3, 2, 1]));
        assert_eq!(Some(&1), middle(&vec![1]));
        assert_eq!(None, middle(&vec![1, 2, 3, 3, 2, 1]));
        assert_eq!(None, middle::<u32>(&vec![]));
    }

    /// Test that fixing updates will correctly swap around things until they
    /// are valid.
    #[test]
    fn test_fix_update_1() {
        let rules = vec![
            (47, 53),
            (97, 13),
            (97, 61),
            (97, 47),
            (75, 29),
            (61, 13),
            (75, 53),
            (29, 13),
            (97, 29),
            (53, 29),
            (61, 53),
            (97, 53),
            (61, 29),
            (47, 13),
            (75, 47),
            (97, 75),
            (47, 61),
            (75, 61),
            (47, 29),
            (75, 13),
            (53, 13)
        ];
        let update = vec![75, 97, 47, 61, 53];
        let fixed = fix_update(&rules, &update);
        assert_eq!(fixed, vec![97, 75, 47, 61, 53])
    }

    /// Test that fixing updates will correctly swap around things until they
    /// are valid.
    #[test]
    fn test_fix_update_2() {
        let rules = vec![
            (47, 53),
            (97, 13),
            (97, 61),
            (97, 47),
            (75, 29),
            (61, 13),
            (75, 53),
            (29, 13),
            (97, 29),
            (53, 29),
            (61, 53),
            (97, 53),
            (61, 29),
            (47, 13),
            (75, 47),
            (97, 75),
            (47, 61),
            (75, 61),
            (47, 29),
            (75, 13),
            (53, 13)
        ];
        let update = vec![61, 13, 29];
        let fixed = fix_update(&rules, &update);
        assert_eq!(fixed, vec![61, 29, 13]);
    }

    /// Test that fixing updates will correctly swap around things until they
    /// are valid.
    #[test]
    fn test_fix_update_3() {
        let rules = vec![
            (47, 53),
            (97, 13),
            (97, 61),
            (97, 47),
            (75, 29),
            (61, 13),
            (75, 53),
            (29, 13),
            (97, 29),
            (53, 29),
            (61, 53),
            (97, 53),
            (61, 29),
            (47, 13),
            (75, 47),
            (97, 75),
            (47, 61),
            (75, 61),
            (47, 29),
            (75, 13),
            (53, 13)
        ];
        let update = vec![97, 13, 75, 29, 47];
        let fixed = fix_update(&rules, &update);
        assert_eq!(fixed, vec![97, 75, 47, 29, 13]);
    }

    /// Test that fixing updates will correctly swap around things until they
    /// are valid.
    #[test]
    fn test_fix_update_4() {
        let rules = vec![
            (47, 53),
            (97, 13),
            (97, 61),
            (97, 47),
            (75, 29),
            (61, 13),
            (75, 53),
            (29, 13),
            (97, 29),
            (53, 29),
            (61, 53),
            (97, 53),
            (61, 29),
            (47, 13),
            (75, 47),
            (97, 75),
            (47, 61),
            (75, 61),
            (47, 29),
            (75, 13),
            (53, 13)
        ];
        let update = vec![75, 47, 61, 53, 29];
        let fixed = fix_update(&rules, &update);
        assert_eq!(fixed, vec![75, 47, 61, 53, 29]);
    }

}
