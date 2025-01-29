// --- Part Two ---
//
// The engineers seem concerned; the total calibration result you gave them is
// nowhere close to being within safety tolerances. Just then, you spot your
// mistake: some well-hidden elephants are holding a third type of operator.
//
// The concatenation operator (||) combines the digits from its left and right
// inputs into a single number. For example, 12 || 345 would become 12345. All
// operators are still evaluated left-to-right.
//
// Now, apart from the three equations that could be made true using only
// addition and multiplication, the above example has three more equations that
// can be made true by inserting operators:
//
// * 156: 15 6 can be made true through a single concatenation: 15 || 6 = 156.
// * 7290: 6 8 6 15 can be made true using 6 * 8 || 6 * 15.
// * 192: 17 8 14 can be made true using 17 || 8 + 14.
//
// Adding up all six test values (the three that could be made before using only
// + and * plus the new three that can now be made by also using ||) produces
// the new total calibration result of 11387.
//
// Using your new knowledge of elephant hiding spots, determine which equations
// could possibly be true. What is their total calibration result?
use std::io;
use std::io::BufRead;
use std::fmt::Debug;
use aoc2024::aoc::AocError;
use aoc2024::nom_helpers::parse_u64;
use nom::IResult;
use nom::multi::separated_list1;
use nom::character::complete::char;
use nom::combinator::all_consuming;

fn main() -> Result<(), AocError> {
    let stdin = io::stdin();
    let lock = stdin.lock();
    let mut sum = 0;
    for line_or_err in lock.lines() {
        let line = line_or_err?;
        let equation = parse_equation(&line)?;
        if equation.has_solution() {
            sum = sum + equation.result;
        }
    }

    println!("{:?}", sum);
    return Ok(());
}

/// An equation without operators.
#[derive(Debug, PartialEq, Eq, Clone)]
struct Equation {

    /// The operands just waiting for operators to operate on them.
    operands: Vec<u64>,

    /// Supposed result.
    result: u64

}

impl Equation {

    /// Test whether any solution exist to the equation.
    fn has_solution(&self) -> bool {
        if self.operands.len() == 0 {
            return self.result == 0;
        } else {
            return self.test(1, self.operands[0]);
        }
    }

    /// Recursive helper function for testing whether solutions exist.
    fn test(&self, i: usize, sum: u64) -> bool {
        if i == self.operands.len() {
            return sum == self.result;
        }

        let n = self.operands[i];
        return self.test(i + 1, sum + n) ||
            self.test(i + 1, sum * n) ||
            self.test(i + 1, concatenate(sum, n));
    }

}

/// Parse an equation from the given string or error.
fn parse_equation(equation: &str) -> Result<Equation, AocError> {
    let (_, parsed) = all_consuming(do_parse_equation)(equation)?;
    return Ok(parsed);
}

/// Parse an equation from the given string or error.
fn do_parse_equation(input: &str) -> IResult<&str, Equation> {
    let (input, result) = parse_u64(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, ns) = separated_list1(char(' '), parse_u64)(input)?;

    return Ok((input, Equation {
        operands: ns,
        result: result
    }));
}

/// Concatenate the second integer behind the first in base 10.
fn concatenate(a: u64, b: u64) -> u64 {
    let b_chars = b.to_string().len().try_into().unwrap();
    return a * 10_u64.pow(b_chars) + b;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that parsing an equation works as expected.
    #[test]
    fn test_parse_equation_1() {
        let input = "123: 1 2 3";
        let equation = parse_equation(input).unwrap();
        let expected = Equation {
            operands: vec![1, 2, 3],
            result: 123
        };
        assert_eq!(equation, expected);
    }

    /// Test that parsing an equation works as expected.
    #[test]
    fn test_parse_equation_2() {
        let input = "123: 1 2 3whateverkj;w";
        let error = parse_equation(input);
        assert!(error.is_err());
    }

    /// Test that parsing an equation works as expected.
    #[test]
    fn test_parse_equation_3() {
        let input = "123: 1 21sf 3";
        let error = parse_equation(input);
        assert!(error.is_err());
    }

    /// Test that evaluating equations works as expected.
    #[test]
    fn test_has_solution_1() {
        let equation = parse_equation("190: 10 19").unwrap();
        assert!(equation.has_solution());
    }

    /// Test that evaluating equations works as expected.
    #[test]
    fn test_has_solution_2() {
        let equation = parse_equation("3267: 81 40 27").unwrap();
        assert!(equation.has_solution());
    }

    /// Test that evaluating equations works as expected.
    #[test]
    fn test_has_solution_3() {
        let equation = parse_equation("83: 17 5").unwrap();
        assert!(!equation.has_solution());
    }

    /// Test that evaluating equations works as expected.
    #[test]
    fn test_has_solution_4() {
        let equation = parse_equation("156: 15 6").unwrap();
        assert!(equation.has_solution());
    }

    /// Test that evaluating equations works as expected.
    #[test]
    fn test_has_solution_5() {
        let equation = parse_equation("7290: 6 8 6 15").unwrap();
        assert!(equation.has_solution());
    }

    /// Test that evaluating equations works as expected.
    #[test]
    fn test_has_solution_6() {
        let equation = parse_equation("161011: 16 10 13").unwrap();
        assert!(!equation.has_solution());
    }

    /// Test that evaluating equations works as expected.
    #[test]
    fn test_has_solution_7() {
        let equation = parse_equation("192: 17 8 14").unwrap();
        assert!(equation.has_solution());
    }

    /// Test that evaluating equations works as expected.
    #[test]
    fn test_has_solution_8() {
        let equation = parse_equation("21037: 9 7 18 13").unwrap();
        assert!(!equation.has_solution());
    }

    /// Test that evaluating equations works as expected.
    #[test]
    fn test_has_solution_9() {
        let equation = parse_equation("292: 11 6 16 20").unwrap();
        assert!(equation.has_solution());
    }

    /// Test that concatenating integers works.
    #[test]
    fn test_concatenate() {
        assert_eq!(100, concatenate(0, 100));
        assert_eq!(123456789, concatenate(1234, 56789));
    }

}
