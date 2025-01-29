// --- Day 7: Bridge Repair ---
//
// The Historians take you to a familiar rope bridge over a river in the middle
// of a jungle. The Chief isn't on this side of the bridge, though; maybe he's
// on the other side?
//
// When you go to cross the bridge, you notice a group of engineers trying to
// repair it. (Apparently, it breaks pretty frequently.) You won't be able to
// cross until it's fixed.
//
// You ask how long it'll take; the engineers tell you that it only needs final
// calibrations, but some young elephants were playing nearby and stole all the
// operators from their calibration equations! They could finish the
// calibrations if only someone could determine which test values could possibly
// be produced by placing any combination of operators into their calibration
// equations (your puzzle input).
//
// For example:
//
//    190: 10 19
//    3267: 81 40 27
//    83: 17 5
//    156: 15 6
//    7290: 6 8 6 15
//    161011: 16 10 13
//    192: 17 8 14
//    21037: 9 7 18 13
//    292: 11 6 16 20
//
// Each line represents a single equation. The test value appears before the
// colon on each line; it is your job to determine whether the remaining numbers
// can be combined with operators to produce the test value.
//
// Operators are always evaluated left-to-right, not according to precedence
// rules. Furthermore, numbers in the equations cannot be rearranged. Glancing
// into the jungle, you can see elephants holding two different types of
// operators: add (+) and multiply (*).
//
// Only three of the above equations can be made true by inserting operators:
//
// * 190: 10 19 has only one position that accepts an operator: between 10 and
//   19. Choosing + would give 29, but choosing * would give the test value (10
//   * 19 = 190).
// * 3267: 81 40 27 has two positions for operators. Of the four possible
//   configurations of the operators, two cause the right side to match the test
//   value: 81 + 40 * 27 and 81 * 40 + 27 both equal 3267 (when evaluated
//   left-to-right)!
// * 292: 11 6 16 20 can be solved in exactly one way: 11 + 6 * 16 + 20.
//
// The engineers just need the total calibration result, which is the sum of the
// test values from just the equations that could possibly be true. In the above
// example, the sum of the test values for the three equations listed above is
// 3749.
//
// Determine which equations could possibly be true. What is their total
// calibration result?
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
        let has_solution = equation.has_solution()?;
        if has_solution {
            sum = sum + equation.result;
        }
    }

    println!("{:?}", sum);
    return Ok(());
}

/// An equation without operators.
#[derive(Debug, PartialEq, Eq)]
struct Equation {

    /// The operands just waiting for operators to operate on them.
    operands: Vec<u64>,

    /// Supposed result.
    result: u64

}

impl Equation {

    /// Test whether any assignment of operators can make the equation true.
    fn has_solution(&self) -> Result<bool, AocError> {
        let operator_count: u32 = (self.operands.len() - 1).try_into().unwrap();
        if operator_count > 32 {
            return Err(AocError::UnexpectedError(
                    "Equation longer than supported length.".to_string()));
        }
        for mask in 0..2_u32.pow(operator_count) {
            if self.result == self.compute_result(mask) {
                return Ok(true);
            }
        }

        return Ok(false);
    }

    /// Compute the result of the equation using the given mask.
    ///
    /// The mask is used to determine whether to use add or multiply as the
    /// operation.
    fn compute_result(&self, mask: u32) -> u64 {
        if self.operands.len() == 0 {
            return 0;
        }
        let mut result: u64 = self.operands[0];
        for i in 1..self.operands.len() {
            let n: u64 = self.operands[i];
            if nth_bit(mask, (i - 1).try_into().unwrap()) {
                result = result + n;
            } else {
                result = result * n;
            }
        }

        return result;
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

/// Test whether the n'th least significant bit is 1.
fn nth_bit(value: u32, n: u32) -> bool {
    (value >> n) & 1 == 1
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
        assert_eq!(Ok(true), equation.has_solution());
    }

    /// Test that evaluating equations works as expected.
    #[test]
    fn test_has_solution_2() {
        let equation = parse_equation("3267: 81 40 27").unwrap();
        assert_eq!(Ok(true), equation.has_solution());
    }

    /// Test that evaluating equations works as expected.
    #[test]
    fn test_has_solution_3() {
        let equation = parse_equation("83: 17 5").unwrap();
        assert_eq!(Ok(false), equation.has_solution());
    }

    /// Test that evaluating equations works as expected.
    #[test]
    fn test_has_solution_4() {
        let equation = parse_equation("156: 15 6").unwrap();
        assert_eq!(Ok(false), equation.has_solution());
    }

    /// Test that evaluating equations works as expected.
    #[test]
    fn test_has_solution_5() {
        let equation = parse_equation("7290: 6 8 6 15").unwrap();
        assert_eq!(Ok(false), equation.has_solution());
    }

    /// Test that evaluating equations works as expected.
    #[test]
    fn test_has_solution_6() {
        let equation = parse_equation("161011: 16 10 13").unwrap();
        assert_eq!(Ok(false), equation.has_solution());
    }

    /// Test that evaluating equations works as expected.
    #[test]
    fn test_has_solution_7() {
        let equation = parse_equation("192: 17 8 14").unwrap();
        assert_eq!(Ok(false), equation.has_solution());
    }

    /// Test that evaluating equations works as expected.
    #[test]
    fn test_has_solution_8() {
        let equation = parse_equation("21037: 9 7 18 13").unwrap();
        assert_eq!(Ok(false), equation.has_solution());
    }

    /// Test that evaluating equations works as expected.
    #[test]
    fn test_has_solution_9() {
        let equation = parse_equation("292: 11 6 16 20").unwrap();
        assert_eq!(Ok(true), equation.has_solution());
    }

}
