// --- Day 5: Print Queue ---
//
// Satisfied with their search on Ceres, the squadron of scholars suggests
// subsequently scanning the stationery stacks of sub-basement 17.
//
// The North Pole printing department is busier than ever this close to
// Christmas, and while The Historians continue their search of this
// historically significant facility, an Elf operating a very familiar printer
// beckons you over.
//
// The Elf must recognize you, because they waste no time explaining that the
// new sleigh launch safety manual updates won't print correctly. Failure to
// update the safety manuals would be dire indeed, so you offer your services.
//
// Safety protocols clearly indicate that new pages for the safety manuals must
// be printed in a very specific order. The notation X|Y means that if both page
// number X and page number Y are to be produced as part of an update, page
// number X must be printed at some point before page number Y.
//
// The Elf has for you both the page ordering rules and the pages to produce in
// each update (your puzzle input), but can't figure out whether each update has
// the pages in the right order.
//
// For example:
//
//    47|53
//    97|13
//    97|61
//    97|47
//    75|29
//    61|13
//    75|53
//    29|13
//    97|29
//    53|29
//    61|53
//    97|53
//    61|29
//    47|13
//    75|47
//    97|75
//    47|61
//    75|61
//    47|29
//    75|13
//    53|13
//
//    75,47,61,53,29
//    97,61,53,29,13
//    75,29,13
//    75,97,47,61,53
//    61,13,29
//    97,13,75,29,47
//
// The first section specifies the page ordering rules, one per line. The first
// rule, 47|53, means that if an update includes both page number 47 and page
// number 53, then page number 47 must be printed at some point before page
// number 53. (47 doesn't necessarily need to be immediately before 53; other
// pages are allowed to be between them.)
//
// The second section specifies the page numbers of each update. Because most
// safety manuals are different, the pages needed in the updates are different
// too. The first update, 75,47,61,53,29, means that the update consists of page
// numbers 75, 47, 61, 53, and 29.
//
// To get the printers going as soon as possible, start by identifying which
// updates are already in the right order.
//
// In the above example, the first update (75,47,61,53,29) is in the right
// order:
//
// * 75 is correctly first because there are rules that put each other page
//   after it: 75|47, 75|61, 75|53, and 75|29.
// * 47 is correctly second because 75 must be before it (75|47) and every other
//   page must be after it according to 47|61, 47|53, and 47|29.
// * 61 is correctly in the middle because 75 and 47 are before it (75|61 and
//   47|61) and 53 and 29 are after it (61|53 and 61|29).
// * 53 is correctly fourth because it is before page number 29 (53|29).
// * 29 is the only page left and so is correctly last.
//
// Because the first update does not include some page numbers, the ordering
// rules involving those missing page numbers are ignored.
//
// The second and third updates are also in the correct order according to the
// rules. Like the first update, they also do not include every page number, and
// so only some of the ordering rules apply - within each update, the ordering
// rules that involve missing page numbers are not used.
//
// The fourth update, 75,97,47,61,53, is not in the correct order: it would
// print 75 before 97, which violates the rule 97|75.
//
// The fifth update, 61,13,29, is also not in the correct order, since it breaks
// the rule 29|13.
//
// The last update, 97,13,75,29,47, is not in the correct order due to breaking
// several rules.
//
// For some reason, the Elves also need to know the middle page number of each
// update being printed. Because you are currently only printing the
// correctly-ordered updates, you will need to find the middle page number of
// each correctly-ordered update. In the above example, the correctly-ordered
// updates are:
//
//     75,47,61,53,29
//     97,61,53,29,13
//     75,29,13
//
// These have middle page numbers of 61, 53, and 29 respectively. Adding these
// page numbers together gives 143.
//
// Of course, you'll need to be careful: the actual list of page ordering rules
// is bigger and more complicated than the above example.
//
// Determine which updates are already in the correct order. What do you get if
// you add up the middle page number from those correctly-ordered updates?
use std::io;
use std::io::Read;
use std::collections::HashSet;
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
    let rules = compute_rules(&input);
    let mut sum = 0;
    for update in input.updates {
        if verify_update(&rules, &update) {
            let middle = middle(&update)
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

fn compute_rules(input: &Input) -> HashMap<Page, HashSet<Page>> {
    let mut rules = HashMap::new();
    for (before, after) in &input.rules {
        rules
            .entry(*before)
            .and_modify(|rule: &mut HashSet<Page>| {
                rule.insert(*after);
            })
            .or_insert(HashSet::from([*after]));
    }

    return rules;
}

fn verify_update(rules: &HashMap<Page, HashSet<Page>>, update: &Update) -> bool {
    let mut seen = HashSet::new();
    for page in update {
        seen.insert(*page);
        match rules.get(&page) {
            Some(disallowed) => {
                if seen.intersection(disallowed).next().is_some() {
                    return false;
                }
            },
            None => {}
        }
    }

    return true;
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

    /// Test that computing rules works as expected.
    #[test]
    fn test_compute_rules_1() {
        let input = Input {
            rules: vec![],
            updates: vec![]
        };
        assert_eq!(HashMap::new(), compute_rules(&input));
    }

    /// Test that computing rules works as expected.
    #[test]
    fn test_compute_rules_2() {
        let input = Input {
            rules: vec![(1, 2), (3, 4), (1, 4)],
            updates: vec![]
        };
        let mut expected = HashMap::new();
        expected.insert(1, HashSet::from([2, 4]));
        expected.insert(3, HashSet::from([4]));

        assert_eq!(expected, compute_rules(&input));
    }

    /// Test that verifying an update works as expected.
    #[test]
    fn test_verify_update() {
        let mut rules = HashMap::new();
        assert!(verify_update(&rules, &vec![1, 2, 3, 4]));

        rules.insert(1, HashSet::from([2, 3, 4]));
        assert!(verify_update(&rules, &vec![1, 2, 3, 4]));

        rules.insert(2, HashSet::from([3, 4]));
        assert!(verify_update(&rules, &vec![1, 2, 3, 4]));

        rules.insert(3, HashSet::from([2]));
        assert!(!verify_update(&rules, &vec![1, 2, 3, 4]));
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

}
