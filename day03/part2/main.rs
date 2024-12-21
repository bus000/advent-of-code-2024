// --- Part Two ---
//
// As you scan through the corrupted memory, you notice that some of the
// conditional statements are also still intact. If you handle some of the
// uncorrupted conditional statements in the program, you might be able to get
// an even more accurate result.
//
// There are two new instructions you'll need to handle:
//
//    * The do() instruction enables future mul instructions.
//    * The don't() instruction disables future mul instructions.
//
// Only the most recent do() or don't() instruction applies. At the beginning of
// the program, mul instructions are enabled.
//
// For example:
//
//     xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
//
// This corrupted memory is similar to the example from before, but this time
// the mul(5,5) and mul(11,8) instructions are disabled because there is a
// don't() instruction before them. The other mul instructions function
// normally, including the one at the end that gets re-enabled by a do()
// instruction.
//
// This time, the sum of the results is 48 (2*4 + 8*5).
//
// Handle the new instructions; what do you get if you add up all of the results
// of just the enabled multiplications?
use std::io;
use regex::Regex;

fn main() {
    let mut total_sum = 0;
    for line in io::stdin().lines() {
        total_sum = total_sum + sum_of_muls(&line.unwrap());
    }

    println!("{:?}", total_sum);
}

/// A runner capable of running a program.
///
/// A runner consumes one byte at a time. It contains a state machine that
/// parses instructions and throws away garbage. Whenever a terminal state in
/// the state machine is reached an instruction has been parsed and it is
/// executed.
struct ProgramRunner {

    /// Current state of the state machine.
    current_state : mut State,

    /// The current sum of all mul instructions.
    running_sum : mut u32,

    /// Whether mul instructions is currently enabled.
    mul_enabled : bool

}

impl ProgramRunner {

    fn new() -> ProgramRunner {
        ProgramRunner {
            current_state = /* TODO */,
            running_sum = 0,
            mul_enabled = true
        }
    }

    fn consume_byte(&self, byte : u8) {
        self.current_state.transition(byte);
        match self.current_state {
            
        }
    }

}

// Extract the multiplications, compute them and return the sum.
fn sum_of_muls(string : &str) -> u32 {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut product_sum = 0;
    for (_, [n1_str, n2_str]) in re.captures_iter(string).map(|c| c.extract()) {
        let n1 = u32::from_str_radix(n1_str, 10).unwrap();
        let n2 = u32::from_str_radix(n2_str, 10).unwrap();
        product_sum = product_sum + n1 * n2;
    }

    return product_sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that we don't crash on empty strings.
    #[test]
    fn test_sum_of_muls_empty() {
        assert_eq!(0, sum_of_muls(""));
    }

    /// Test that we don't make up phantom muls.
    #[test]
    fn test_sum_of_muls_no_phantoms() {
        assert_eq!(0, sum_of_muls("whatever but no muls here. mul mul(123n)"));
    }

    /// Test that we can find a normal mul.
    #[test]
    fn test_sum_of_muls_middle() {
        assert_eq!(20, sum_of_muls("whatever but no muls here. mul(4,5)rsfd"));
    }

    /// Test that we can find a mul even if it is last.
    #[test]
    fn test_sum_of_muls_last() {
        assert_eq!(20, sum_of_muls("whatever but no muls here. mul(4,5)"));
    }

    /// Test that we can find a mul even if it is first.
    #[test]
    fn test_sum_of_muls_first() {
        assert_eq!(20, sum_of_muls("mul(4,5)whatever but no muls here."));
    }

    /// Test that we can find multiple muls and will sum them.
    #[test]
    fn test_sum_of_muls_multiple() {
        assert_eq!(79, sum_of_muls("mul(4,5)whatever mul(1,5) mul(6,9)"));
    }

}
