// --- Part Two ---
// Your analysis only confirmed what everyone feared: the two lists of location
// IDs are indeed very different.
//
// Or are they?
//
// The Historians can't agree on which group made the mistakes or how to read
// most of the Chief's handwriting, but in the commotion you notice an
// interesting detail: a lot of location IDs appear in both lists! Maybe the
// other numbers aren't location IDs at all but rather misinterpreted
// handwriting.
//
// This time, you'll need to figure out exactly how often each number from the
// left list appears in the right list. Calculate a total similarity score by
// adding up each number in the left list after multiplying it by the number of
// times that number appears in the right list.
//
// Here are the same example lists again:
//
//    3   4
//    4   3
//    2   5
//    1   3
//    3   9
//    3   3
//
// For these example lists, here is the process of finding the similarity score:
//
// * The first number in the left list is 3. It appears in the right list three
//   times, so the similarity score increases by 3 * 3 = 9.
// * The second number in the left list is 4. It appears in the right list once,
//   so the similarity score increases by 4 * 1 = 4.
// * The third number in the left list is 2. It does not appear in the right
//   list, so the similarity score does not increase (2 * 0 = 0).
// * The fourth number, 1, also does not appear in the right list.
// * The fifth number, 3, appears in the right list three times; the similarity
//   score increases by 9.
// * The last number, 3, appears in the right list three times; the similarity
//   score again increases by 9.
//
// So, for these example lists, the similarity score at the end of this process
// is 31 (9 + 4 + 0 + 0 + 9 + 9).
//
// Once again consider your left and right lists. What is their similarity
// score?
use std::io;
use std::collections::HashMap;
use aoc2024::nom_helpers::parse_u32;
use nom::{
    Parser,
    IResult,
    multi::many1,
    character::complete::{one_of, char},
    combinator::{map_res, recognize, eof}
};

fn main() {
    let lines = io::stdin().lines();
    let mut numbers = Vec::new();
    let mut counts = HashMap::new();
    for line in lines {
        let (n1, n2) = parse_line(&line.unwrap());
        numbers.push(n1);
        counts.entry(n2).and_modify(|n| *n += 1).or_insert(1);
    }

    let mut similarity = 0;
    for n in numbers.iter() {
        let score = counts.get(n).unwrap_or(&0) * n;
        similarity = similarity + score;
    }

    println!("{:?}", similarity);
}

/// Parse a line from the input.
fn parse_line(input: &str) -> (u32, u32) {
    let (_, ns) = do_parse_line(input).unwrap();
    return ns;
}

/// Parse a line from the input.
fn do_parse_line(input: &str) -> IResult<&str, (u32, u32)> {
    let (input, n1) = parse_u32(input)?;
    let (input, _) = many1(char(' ')).parse(input)?;
    let (input, n2) = parse_u32(input)?;
    let (input, _) = eof(input)?;

    Ok((input, (n1, n2)))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that parsing a line from the input works.
    #[test]
    fn test_parse_line() {
        let parsed = parse_line("3   4");
        assert_eq!(parsed, (3, 4));
    }

    /// Test that parsing a line from the input fails if line is not valid.
    #[test]
    #[should_panic]
    fn test_parse_line_invalid() {
        parse_line("3   4whatever");
    }

}
